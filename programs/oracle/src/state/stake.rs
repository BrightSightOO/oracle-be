use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct Stake {
    account_type: AccountType,

    /// Owner of the stake.
    pub owner: Pubkey,
    /// Address the stake is delegated to.
    ///
    /// The delegate can vote and restake rewards, but cannot withdraw stake.
    pub delegate: Pubkey,

    /// The amount staked.
    pub amount: u64,

    /// Unix timestamp at which this stake will allow withdrawal.
    pub lockup: i64,
}

impl Account for Stake {
    const TYPE: AccountType = AccountType::Stake;
}
