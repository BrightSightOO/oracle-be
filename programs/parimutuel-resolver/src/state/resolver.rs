use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct Resolver {
    account_type: AccountType,

    /// Parimutuel market.
    pub market: Pubkey,
    /// Oracle request to source outcome.
    pub request: Pubkey,
}

impl Account for Resolver {
    const TYPE: AccountType = AccountType::Resolver;
}

impl From<InitResolver> for (Resolver, usize) {
    fn from(params: InitResolver) -> (Resolver, usize) {
        let InitResolver { market, request } = params;

        (Resolver { account_type: Resolver::TYPE, market, request }, Resolver::SIZE)
    }
}

pub(crate) struct InitResolver {
    pub market: Pubkey,
    pub request: Pubkey,
}
