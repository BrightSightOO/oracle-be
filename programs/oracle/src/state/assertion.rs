use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, ShankAccount, BorshSize)]
pub struct Assertion {
    account_type: AccountType,

    /// The [`Request`]` this assertion is for.
    ///
    /// [`Request`]: crate::state::Request
    pub request: Pubkey,

    /// Amount that the asserter has bonded.
    pub bond: u64,
    /// Bond mint.
    pub bond_mint: Pubkey,

    /// Unix timestamp of the assertion.
    pub assertion_timestamp: i64,
    /// Unix timestamp at which the assertion can no longer be disputed.
    pub expiration_timestamp: i64,

    /// Asserter address.
    pub asserter: Pubkey,
    /// Disputer address.
    pub disputer: Pubkey,

    /// Value submitted by the asserter.
    pub proposed_value: u64,
    /// Value of the resolved request.
    pub resolved_value: u64,
}

impl Account for Assertion {
    const TYPE: AccountType = AccountType::Assertion;
}
