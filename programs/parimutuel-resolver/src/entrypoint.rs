use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::PrintProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::ResolverError;
use crate::processor;

solana_program::entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        error.print::<ResolverError>();
        return Err(error);
    }
    Ok(())
}
