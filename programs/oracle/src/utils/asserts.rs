use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

pub fn assert_token_program(pubkey: &Pubkey) -> Result<(), ProgramError> {
    if !common::cmp_pubkeys(pubkey, &cpi::spl::TOKEN_ID)
        && !common::cmp_pubkeys(pubkey, &cpi::spl::TOKEN_2022_ID)
    {
        log!("Error: Incorrect address for token program");
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

pub fn assert_system_program(pubkey: &Pubkey) -> Result<(), ProgramError> {
    if !common::cmp_pubkeys(pubkey, &system_program::ID) {
        log!("Error: Incorrect address for system program");
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

pub fn assert_signer(account_info: &AccountInfo) -> Result<(), ProgramError> {
    if !account_info.is_signer {
        log!("Error: Account {} is expected to be a signer", account_info.key);
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}
