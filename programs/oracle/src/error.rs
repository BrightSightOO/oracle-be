use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Error, FromPrimitive)]
pub enum OracleError {
    #[error("Program arithmetic overflowed")]
    ArithmeticOverflow,
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
