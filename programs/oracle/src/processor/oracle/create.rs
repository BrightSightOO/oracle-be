use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::{Context, CreateOracleAccounts};
use crate::instruction::CreateOracleArgs;
use crate::state::{InitAccount, InitContext, InitOracle, Oracle};
use crate::{pda, utils};

pub fn create<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateOracleArgs,
) -> ProgramResult {
    let ctx = CreateOracleAccounts::context(accounts)?;

    match args {
        CreateOracleArgs::V1 { .. } => create_v1(program_id, ctx, args),
    }
}

fn create_v1(
    program_id: &Pubkey,
    ctx: Context<CreateOracleAccounts>,
    args: CreateOracleArgs,
) -> ProgramResult {
    let CreateOracleArgs::V1 { authority } = args;

    let CreateOracleAccounts { oracle, payer, system_program } = ctx.accounts;

    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    utils::assert_system_program(system_program.key)?;

    let oracle_bump = pda::oracle::assert_pda(oracle.key)?;
    let signer_seeds = pda::oracle::seeds_with_bump(&oracle_bump);

    // Step 1: Initialize `oracle` account.
    Oracle::init(InitOracle { authority }).save(InitContext {
        account: oracle,
        payer,
        system_program,
        program_id,
        signer_seeds: &[&signer_seeds],
    })?;

    Ok(())
}
