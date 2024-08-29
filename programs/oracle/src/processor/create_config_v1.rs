use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::CreateConfigV1Accounts;
use crate::state::{ConfigV1, InitAccount, InitConfig, InitContext};
use crate::utils;

#[derive(Clone, BorshDeserialize)]
pub struct CreateConfigV1Args {
    pub authority: Pubkey,

    // TODO: Validate fee bps.
    pub bond_fee_bps: u16,

    pub dispute_window: u32,
    pub voting_window: u32,
    pub arbitration_window: u32,
}

pub fn create_config_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateConfigV1Args,
) -> ProgramResult {
    let ctx = CreateConfigV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Step 1: Initialize `config` account.
    {
        ConfigV1::init(InitConfig {
            authority: args.authority,
            bond_fee_bps: args.bond_fee_bps,
            dispute_window: args.dispute_window,
            voting_window: args.voting_window,
            arbitration_window: args.arbitration_window,
        })
        .save(InitContext {
            account: ctx.accounts.config,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[],
        })?;
    }

    Ok(())
}
