use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::CreateOracleV1Accounts;
use crate::state::{InitAccount, InitContext, InitOracle, OracleV1};
use crate::{pda, utils};

#[derive(Clone, BorshDeserialize)]
pub struct CreateOracleV1Args {
    pub authority: Pubkey,
}

pub fn create_oracle_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateOracleV1Args,
) -> ProgramResult {
    let ctx = CreateOracleV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Step 1: Check governance mint exists.
    cpi::spl::mint_decimals(ctx.accounts.governance_mint)?;

    // Step 2: Initialize `oracle` account.
    {
        let bump = pda::oracle::assert_pda(ctx.accounts.oracle.key)?;
        let signer_seeds = pda::oracle::seeds_with_bump(&bump);

        OracleV1::init(InitOracle {
            authority: args.authority,
            governance_mint: *ctx.accounts.governance_mint.key,
        })
        .save(InitContext {
            account: ctx.accounts.oracle,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    Ok(())
}
