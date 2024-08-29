use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::clock::UnixTimestamp;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;

use super::{Account, AccountType};

// TODO:
// - unstaking
// - merging staking accounts
//   - cannot be done if an account has a lock for an active vote

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct StakeV1 {
    account_type: AccountType,

    /// The mint address.
    pub mint: Pubkey,
    /// The owner address.
    pub owner: Pubkey,
    /// The address the stake is delegated to.
    ///
    /// The delegate can vote and restake rewards, but cannot withdraw stake.
    pub delegate: Pubkey,

    /// The amount staked.
    pub amount: u64,

    /// The Unix timestamp the stake is locked until.
    pub lock_timestamp: UnixTimestamp,
}

impl StakeV1 {
    pub fn assert_voter(&self, voter: &Pubkey) -> Result<(), OracleError> {
        if !common::cmp_pubkeys(&self.owner, voter) && !common::cmp_pubkeys(&self.delegate, voter) {
            return Err(OracleError::StakeVoterMismatch);
        }
        Ok(())
    }
}

impl Account for StakeV1 {
    const TYPE: AccountType = AccountType::StakeV1;
}

impl From<InitStake> for (StakeV1, usize) {
    fn from(params: InitStake) -> (StakeV1, usize) {
        let InitStake { mint, owner, amount } = params;

        (
            StakeV1 {
                account_type: StakeV1::TYPE,
                mint,
                owner,
                delegate: owner,
                amount,
                lock_timestamp: UnixTimestamp::MIN,
            },
            StakeV1::SIZE,
        )
    }
}

pub(crate) struct InitStake {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}
