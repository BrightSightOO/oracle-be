use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Error, FromPrimitive)]
pub enum OracleError {
    #[error("Failed to deserialize account")]
    DeserializationError,

    #[error("Failed to serialize account")]
    SerializationError,

    #[error("Reward must be within valid bounds")]
    RewardBounds,

    #[error("Bond must be within valid bounds")]
    BondBounds,

    #[error("Value is not valid for the request")]
    InvalidValue,

    #[error("Disputer cannot be the same as the asserter")]
    DisputerIsAsserter,

    #[error("Request does not have an assertion")]
    NotAsserted,

    #[error("Request is not disputed")]
    NotDisputed,

    #[error("Request already has an assertion")]
    AlreadyAsserted,

    #[error("Assertion has already been disputed")]
    AlreadyDisputed,

    #[error("Request has already been resolved")]
    AlreadyResolved,

    #[error("Request is not accepting assertion yet")]
    AssertionTooEarly,

    #[error("Dispute window has not expired")]
    DisputeWindowNotExpired,

    #[error("Dispute window has expired")]
    DisputeWindowExpired,

    #[error("Voting window has not expired")]
    VotingWindowNotExpired,

    #[error("Voting window has expired")]
    VotingWindowExpired,

    #[error("Arbitration window has not expired")]
    ArbitrationWindowNotExpired,

    #[error("Oracle authority address does not match")]
    OracleAuthorityMismatch,

    #[error("Config authority address does not match")]
    ConfigAuthorityMismatch,

    #[error("Config address does not match")]
    ConfigMismatch,

    #[error("Currency mint address does not match")]
    CurrencyMintMismatch,

    #[error("Bond mint address does not match")]
    BondMintMismatch,

    #[error("Stake mint address does not match")]
    StakeMintMismatch,

    #[error("Stake delegate does not match voter")]
    StakeVoterMismatch,
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
