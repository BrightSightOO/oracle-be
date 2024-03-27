use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

pub fn assert_token_program(pubkey: &Pubkey) -> Result<(), ProgramError> {
    if !common::cmp_pubkeys(pubkey, &spl_token::ID)
        && !common::cmp_pubkeys(pubkey, &spl_token_2022::ID)
    {
        err!("Incorrect address for token program");
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

pub fn assert_system_program(pubkey: &Pubkey) -> Result<(), ProgramError> {
    if !common::cmp_pubkeys(pubkey, &system_program::ID) {
        err!("Incorrect address for system program");
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
