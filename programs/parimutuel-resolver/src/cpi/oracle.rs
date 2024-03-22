use borsh::BorshDeserialize;
use num_traits::FromPrimitive;
use oracle::state::{AccountType, Request};
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;

pub fn request_from_account_info(info: &AccountInfo) -> Result<Request, ProgramError> {
    if !common::cmp_pubkeys(info.owner, &oracle::ID) {
        err!("Request account is not owned by the oracle program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let data = info.try_borrow_data()?;

    let account_type = data.first().ok_or_else(|| {
        err!("Request account is uninitialized");
        ProgramError::UninitializedAccount
    })?;

    let account_type = AccountType::from_u8(*account_type).ok_or_else(|| {
        err!("Unknown account type: {account_type:#x}, expected Request");
        ProgramError::InvalidAccountData
    })?;

    if account_type != AccountType::Request {
        err!("Incorrect account type: expected Request, found {account_type:?}");
        return match account_type {
            AccountType::Uninitialized => Err(ProgramError::UninitializedAccount),
            _ => Err(ProgramError::InvalidAccountData),
        };
    }

    Request::deserialize(&mut &data[..]).map_err(|err| {
        err!("Failed to deserialize {account_type:?} account: {err}");
        ProgramError::InvalidAccountData
    })
}
