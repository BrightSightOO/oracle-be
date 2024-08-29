use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use borsh_size::{BorshSize, BorshSizeProperties};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSchema, BorshSize, ShankAccount)]
pub struct OracleV1 {
    account_type: AccountType,

    /// Index for the next request.
    pub next_index: u64,

    /// Authority address.
    pub authority: Pubkey,
    /// Governance token mint address.
    pub governance_mint: Pubkey,
}

impl OracleV1 {
    pub fn assert_authority(&self, authority: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(&self.authority, authority) {
            return Err(OracleError::OracleAuthorityMismatch);
        }
        Ok(())
    }
}

impl Account for OracleV1 {
    const TYPE: AccountType = AccountType::OracleV1;
}

impl From<InitOracle> for (OracleV1, usize) {
    fn from(params: InitOracle) -> (OracleV1, usize) {
        let InitOracle { authority, governance_mint } = params;

        (
            OracleV1 { account_type: OracleV1::TYPE, next_index: 0, authority, governance_mint },
            OracleV1::FIXED_SIZE,
        )
    }
}

pub(crate) struct InitOracle {
    pub authority: Pubkey,
    pub governance_mint: Pubkey,
}
