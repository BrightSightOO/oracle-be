use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::BorshSize;
use shank::ShankAccount;
use solana_program::clock::UnixTimestamp;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;
use crate::pda;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct RequestV1 {
    account_type: AccountType,

    /// Index of the request in the oracle.
    pub index: u64,

    /// Config address.
    pub config: Pubkey,
    /// Creator address.
    pub creator: Pubkey,

    /// Amount rewarded to the asserter/disputer on resolution.
    pub reward: u64,
    /// Reward mint.
    pub reward_mint: Pubkey,

    /// Amount required to be bonded by asserter/disputer.
    pub bond: u64,
    /// Bond mint.
    pub bond_mint: Pubkey,

    /// Unix timestamp after which a value can be asserted.
    pub assertion_timestamp: UnixTimestamp,
    /// Unix timestamp at which the request was resolved.
    pub resolve_timestamp: UnixTimestamp,

    /// Request state.
    pub state: RequestState,
    /// Value of the resolved request.
    pub value: u64,

    /// Arbitrator address.
    ///
    /// The arbitrator has the ability to override the result of voting. This
    /// takes the form of a window after voting in which the result can be
    /// changed.
    ///
    /// If the address is the default pubkey (`11111111111111111111111111111111`),
    /// then the request is considered to have no arbitrator.
    pub arbitrator: Pubkey,

    // Request data may have varying layouts when serialized. It is at the end
    // of the account to avoid interfering with GPA lookups.
    /// Request data.
    pub data: RequestData,
}

#[derive(Clone, Copy, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSize)]
#[repr(u8)]
pub enum RequestState {
    /// Request pending a proposal.
    Requested,
    /// Request with a asserted value awaiting resolution.
    Asserted,
    /// Request with a disputed value awaiting voting resolution.
    Disputed,
    /// Request with a resolved value.
    Resolved,
}

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize)]
pub enum RequestData {
    /// Yes/No request:
    /// - 0 = No
    /// - 1 = Yes
    YesNo {
        /// Question.
        question: String,
    },
}

impl RequestV1 {
    pub fn has_arbitrator(&self) -> bool {
        const DEFAULT_PUBKEY: Pubkey = Pubkey::new_from_array([0; 32]);

        !solana_utils::pubkeys_eq(&self.arbitrator, &DEFAULT_PUBKEY)
    }

    pub fn assert_pda(&self, request: &Pubkey) -> Result<u8, ProgramError> {
        pda::request::assert_pda(request, &self.index)
    }

    pub fn assert_config(&self, config: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(&self.config, config) {
            return Err(OracleError::ConfigMismatch);
        }
        Ok(())
    }

    pub fn assert_reward_mint(&self, mint: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(&self.reward_mint, mint) {
            return Err(OracleError::RewardMintMismatch);
        }
        Ok(())
    }

    pub fn assert_bond_mint(&self, mint: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(&self.bond_mint, mint) {
            return Err(OracleError::BondMintMismatch);
        }
        Ok(())
    }

    pub fn validate_assertion_timestamp(&self, timestamp: i64) -> Result<(), OracleError> {
        if timestamp < self.assertion_timestamp {
            return Err(OracleError::AssertionTooEarly);
        }
        Ok(())
    }
}

impl Account for RequestV1 {
    const TYPE: AccountType = AccountType::RequestV1;
}

impl RequestData {
    pub fn validate_value(&self, value: u64) -> Result<(), OracleError> {
        let valid = match self {
            Self::YesNo { .. } => matches!(value, 0 | 1),
        };
        if valid { Ok(()) } else { Err(OracleError::InvalidValue) }
    }
}

impl TryFrom<InitRequest> for (RequestV1, usize) {
    type Error = ProgramError;

    fn try_from(params: InitRequest) -> Result<(RequestV1, usize), Self::Error> {
        let InitRequest {
            index,
            config,
            creator,
            reward,
            reward_mint,
            bond,
            bond_mint,
            timestamp,
            arbitrator,
            data,
        } = params;

        let account = RequestV1 {
            account_type: RequestV1::TYPE,
            index,
            config,
            creator,
            reward,
            reward_mint,
            bond,
            bond_mint,
            assertion_timestamp: timestamp,
            resolve_timestamp: 0,
            state: RequestState::Requested,
            value: 0,
            arbitrator,
            data,
        };
        let space = account.borsh_size();

        Ok((account, space))
    }
}

pub(crate) struct InitRequest {
    pub index: u64,

    pub config: Pubkey,
    pub creator: Pubkey,

    pub reward: u64,
    pub reward_mint: Pubkey,

    pub bond: u64,
    pub bond_mint: Pubkey,

    pub timestamp: UnixTimestamp,
    pub arbitrator: Pubkey,

    pub data: RequestData,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn data_size() {
        let data = RequestData::YesNo { question: "example question?".to_owned() };

        let expected = data.borsh_size();
        let actual = borsh::object_length(&data).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn account_size() {
        let init = InitRequest {
            index: 0,
            config: Pubkey::new_unique(),
            creator: Pubkey::new_unique(),
            reward: 0,
            reward_mint: Pubkey::new_unique(),
            bond: 0,
            bond_mint: Pubkey::new_unique(),
            timestamp: 0,
            arbitrator: Pubkey::new_unique(),
            data: RequestData::YesNo { question: "another example question?".to_owned() },
        };

        let (request, expected) = <(RequestV1, usize)>::try_from(init).unwrap();
        let actual = borsh::object_length(&request).unwrap();

        assert_eq!(expected, actual);
    }
}
