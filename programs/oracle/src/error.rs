use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Error, FromPrimitive)]
pub enum OracleError {
    /// 0 - Arithmetic overflow.
    #[error("Program arithmetic overflowed")]
    ArithmeticOverflow,

    /// 1 - Insufficient bond.
    #[error("Insufficient bond")]
    InsufficientBond,

    /// 2 - Not asserted.
    #[error("Request does not have an assertion")]
    NotAsserted,

    /// 3 - Not disputed.
    #[error("Request is not disputed")]
    NotDisputed,

    /// 4 - Already asserted.
    #[error("Request already has an assertion")]
    AlreadyAsserted,

    /// 5 - Already disputed.
    #[error("Assertion has already been disputed")]
    AlreadyDisputed,

    /// 6 - Already resolved.
    #[error("Request has already been resolved")]
    AlreadyResolved,

    /// 7 - Request not accepting assertions yet.
    #[error("Request is not accepting assertion yet")]
    AssertionTooEarly,

    /// 8 - Dispute window has not expired.
    #[error("Dispute window has not expired")]
    DisputeWindowOpen,

    /// 9 - Dispute window has expired.
    #[error("Dispute window has expired")]
    DisputeWindowExpired,

    /// 10 - Invalid value.
    #[error("Value is not valid for the request")]
    InvalidValue,

    /// 11 - Invalid dispute.
    #[error("Disputed value falls within range of acceptable deviation for asserted value")]
    InvalidDispute,

    /// 12 - Invalid disputer.
    #[error("Disputer cannot be the same as the asserter")]
    DisputerIsAsserter,

    /// 13 - Bond mint mismatch.
    #[error("Bond mint address does not match")]
    BondMismatch,

    /// 14 - Voting window has not expired.
    #[error("Voting window has not expired")]
    VotingWindowOpen,

    /// 15 - Voting window has expired.
    #[error("Voting window has expired")]
    VotingWindowExpired,
}

impl PrintProgramError for OracleError {
    fn print<E>(&self) {
        log!("Error: {self}");
    }
}

impl From<OracleError> for ProgramError {
    fn from(e: OracleError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for OracleError {
    fn type_of() -> &'static str {
        "OracleError"
    }
}
