use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

use crate::cpi;

pub fn assert_parimutuel_program(pubkey: &Pubkey) -> Result<(), ProgramError> {
    if !common::cmp_pubkeys(pubkey, &cpi::parimutuel::ID) {
        err!("Incorrect address for parimutuel program");
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
