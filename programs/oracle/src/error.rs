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

    /// 6 - Dispute window not expired.
    #[error("Assertion dispute window has not expired")]
    DisputeExpireTooEarly,

    /// 7 - Invalid value.
    #[error("Asserted value is not valid for the request")]
    InvalidValue,
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
