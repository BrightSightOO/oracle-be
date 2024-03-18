use borsh::BorshDeserialize;
use common::VariantName;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

mod oracle;

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    use crate::instruction::OracleInstruction as I;

    let instruction = I::try_from_slice(instruction_data)?;

    log!("Instruction: {}", instruction.variant_name());

    match instruction {
        I::CreateOracle(args) => oracle::create(program_id, accounts, args),
    }
}
