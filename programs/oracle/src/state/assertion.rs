use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, ShankAccount, BorshSize)]
pub struct Assertion {
    account_type: AccountType,

    /// The [`Request`]` this assertion is for.
    ///
    /// [`Request`]: crate::state::Request
    pub request: Pubkey,

    /// Amount of governance tokens that the asserter has bonded.
    pub governance: u64,
    /// Amount of bond tokens that the asserter has bonded.
    pub bond: u64,
    /// Bond token mint.
    pub bond_mint: Pubkey,

    /// Unix timestamp of the assertion.
    pub assertion_timestamp: i64,
    /// Unix timestamp at which the assertion is considered resolved.
    ///
    /// Defaults to `assertion_timestamp` + the resolution period,
    /// however if the assertion is disputed and later resolved this will be
    /// the time at which it was resolved.
    pub expiration_timestamp: i64,

    /// Asserter address.
    pub asserter: Pubkey,
    /// Disputer address.
    pub disputer: Pubkey,

    /// Value submitted by the asserter.
    pub asserted_value: u64,
    /// Value of the resolved request.
    pub resolved_value: u64,
}

impl Account for Assertion {
    const TYPE: AccountType = AccountType::Assertion;
}

impl TryFrom<InitAssertion> for (Assertion, usize) {
    type Error = OracleError;

    fn try_from(params: InitAssertion) -> Result<(Assertion, usize), Self::Error> {
        let InitAssertion {
            request,
            governance,
            bond,
            bond_mint,
            assertion_timestamp,
            asserter,
            asserted_value,
        } = params;

        const DAY_SECS: i64 = 86_400;

        let expiration_timestamp =
            assertion_timestamp.checked_add(DAY_SECS).ok_or(OracleError::ArithmeticOverflow)?;

        Ok((
            Assertion {
                account_type: Assertion::TYPE,
                request,
                governance,
                bond,
                bond_mint,
                assertion_timestamp,
                expiration_timestamp,
                asserter,
                disputer: Pubkey::default(),
                asserted_value,
                resolved_value: 0,
            },
            Assertion::SIZE,
        ))
    }
}

pub(crate) struct InitAssertion {
    pub request: Pubkey,

    pub governance: u64,
    pub bond: u64,
    pub bond_mint: Pubkey,

    pub assertion_timestamp: i64,
    pub asserter: Pubkey,
    pub asserted_value: u64,
}
