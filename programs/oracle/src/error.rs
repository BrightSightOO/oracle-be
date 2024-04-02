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

    /// 3 - Already asserted.
    #[error("Request already has an assertion")]
    AlreadyAsserted,

    /// 4 - Already disputed.
    #[error("Assertion has already been disputed")]
    AlreadyDisputed,

    /// 5 - Already resolved.
    #[error("Request has already been resolved")]
    AlreadyResolved,

    /// 6 - Dispute window not expired.
    #[error("Request is not accepting assertion yet")]
    AssertionTooEarly,

    /// 7 - Dispute window not expired.
    #[error("Dispute window has not expired")]
    DisputeWindowOpen,

    /// 8 - Dispute window has expired.
    #[error("Dispute window has expired")]
    DisputeWindowExpired,

    /// 9 - Invalid value.
    #[error("Value is not valid for the request")]
    InvalidValue,

    /// 10 - Invalid dispute.
    #[error("Disputed value falls within range of acceptable deviation for asserted value")]
    InvalidDispute,

    /// 11 - Invalid disputer.
    #[error("Disputer cannot be the same as the asserter")]
    DisputerIsAsserter,

    /// 11 - Bond mint mismatch.
    #[error("Bond mint address does not match")]
    BondMismatch,
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
