use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct Oracle {
    account_type: AccountType,

    /// Index for the next request.
    pub next_index: u64,

    /// Authority address.
    pub authority: Pubkey,
}

impl Account for Oracle {
    const TYPE: AccountType = AccountType::Oracle;
}

impl From<InitOracle> for (Oracle, usize) {
    fn from(params: InitOracle) -> (Oracle, usize) {
        let InitOracle { authority } = params;

        (Oracle { account_type: Oracle::TYPE, next_index: 0, authority }, Oracle::SIZE)
    }
}

pub(crate) struct InitOracle {
    pub authority: Pubkey,
}
