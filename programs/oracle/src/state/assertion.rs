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

    /// Unix timestamp of the assertion.
    pub assertion_timestamp: i64,
    /// Unix timestamp at which the dispute window expires and the assertion
    /// can be resolved.
    ///
    /// This should be [`assertion_timestamp`] + [`DISPUTE_WINDOW`].
    ///
    /// [`assertion_timestamp`]: Assertion::assertion_timestamp
    /// [`DISPUTE_WINDOW`]: crate::DISPUTE_WINDOW
    pub expiration_timestamp: i64,

    /// Asserter address.
    pub asserter: Pubkey,
    /// Disputer address.
    pub disputer: Pubkey,

    /// Value submitted by the asserter.
    pub asserted_value: u64,
    /// Value submitted by the disputer.
    pub disputed_value: u64,
}

impl Assertion {
    pub fn in_dispute_window(&self, timestamp: i64) -> bool {
        timestamp < self.expiration_timestamp
    }

    pub fn validate_expiration_timestamp(&self, timestamp: i64) -> Result<(), OracleError> {
        if self.in_dispute_window(timestamp) {
            return Err(OracleError::DisputeWindowOpen);
        }
        Ok(())
    }

    pub fn validate_dispute_timestamp(&self, timestamp: i64) -> Result<(), OracleError> {
        if !self.in_dispute_window(timestamp) {
            return Err(OracleError::DisputeWindowExpired);
        }
        Ok(())
    }
}

impl Account for Assertion {
    const TYPE: AccountType = AccountType::Assertion;
}

impl TryFrom<InitAssertion> for (Assertion, usize) {
    type Error = OracleError;

    fn try_from(params: InitAssertion) -> Result<(Assertion, usize), Self::Error> {
        let InitAssertion { request, assertion_timestamp, asserter, asserted_value } = params;

        const DAY_SECS: i64 = 86_400;

        let expiration_timestamp =
            assertion_timestamp.checked_add(DAY_SECS).ok_or(OracleError::ArithmeticOverflow)?;

        Ok((
            Assertion {
                account_type: Assertion::TYPE,
                request,
                assertion_timestamp,
                expiration_timestamp,
                asserter,
                disputer: Pubkey::default(),
                asserted_value,
                disputed_value: 0,
            },
            Assertion::SIZE,
        ))
    }
}

pub(crate) struct InitAssertion {
    pub request: Pubkey,

    pub assertion_timestamp: i64,
    pub asserter: Pubkey,
    pub asserted_value: u64,
}
