use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Error, FromPrimitive)]
pub enum ResolverError {
    /// 0 - Arithmetic overflow.
    #[error("Program arithmetic overflowed")]
    ArithmeticOverflow,

    /// 1 - Invalid request.
    #[error("Oracle request must be yes/no type")]
    InvalidRequest,

    /// 2 - Incorrect request.
    #[error("Incorrect oracle request for resolver")]
    IncorrectRequest,

    /// 3 - Not resolved.
    #[error("Oracle request not yet resolved")]
    NotResolved,
}

impl PrintProgramError for ResolverError {
    fn print<E>(&self) {
        log!("Error: {self}");
    }
}

impl From<ResolverError> for ProgramError {
    fn from(e: ResolverError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ResolverError {
    fn type_of() -> &'static str {
        "ResolverError"
    }
}
