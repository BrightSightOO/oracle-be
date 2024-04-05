use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct Currency {
    account_type: AccountType,

    /// The mint address.
    pub mint: Pubkey,

    /// The minimum bond when creating an [`Assertion`].
    ///
    /// [`Assertion`]: crate::state::Assertion
    pub minimum_bond: u64,
}

impl Account for Currency {
    const TYPE: AccountType = AccountType::Currency;
}
