use borsh::BorshDeserialize;
use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;
use crate::instruction::accounts::CreateStakeV1Accounts;
use crate::state::{Account, InitAccount, InitContext, InitStake, OracleV1, StakeV1};
use crate::{pda, utils};

#[derive(Clone, BorshDeserialize)]
pub struct CreateStakeV1Args {
    pub amount: u64,
}

pub fn create_stake_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateStakeV1Args,
) -> ProgramResult {
    let ctx = CreateStakeV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.stake)?;
    utils::assert_signer(ctx.accounts.wallet)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Guard PDAs.
    pda::oracle::assert_pda(ctx.accounts.oracle.key)?;

    // Step 1: Check governance mint.
    {
        let oracle = OracleV1::from_account_info(ctx.accounts.oracle)?;

        if !common::cmp_pubkeys(&oracle.governance_mint, ctx.accounts.mint.key) {
            return Err(OracleError::StakeMintMismatch.into());
        }
    }

    // Step 2: Create stake account.
    StakeV1::init(InitStake {
        mint: *ctx.accounts.mint.key,
        owner: *ctx.accounts.wallet.key,
        amount: args.amount,
    })
    .save(InitContext {
        account: ctx.accounts.stake,
        payer: ctx.accounts.payer,
        system_program: ctx.accounts.system_program,
        program_id,
        signers_seeds: &[],
    })?;

    // Step 3: Create stake pool account if necessary.
    {
        let bump = pda::stake_pool::assert_pda(ctx.accounts.stake_pool.key, ctx.accounts.mint.key)?;

        if ctx.accounts.stake_pool.data_is_empty() {
            let signer_seeds = pda::stake_pool::seeds_with_bump(ctx.accounts.mint.key, &bump);

            cpi::spl::create_token_account(
                ctx.accounts.oracle.key,
                cpi::spl::CreateTokenAccount {
                    account: ctx.accounts.stake_pool,
                    mint: ctx.accounts.mint,
                    payer: ctx.accounts.payer,
                    token_program: ctx.accounts.token_program,
                    system_program: ctx.accounts.system_program,
                },
                &[&signer_seeds],
            )?;
        }
    }

    // Step 4: Deposit staked amount into escrow account.
    {
        let mint_decimals = cpi::spl::mint_decimals(ctx.accounts.mint)?;

        cpi::spl::transfer_checked(
            args.amount,
            mint_decimals,
            cpi::spl::TransferChecked {
                source: ctx.accounts.stake_source,
                destination: ctx.accounts.stake_pool,
                mint: ctx.accounts.mint,
                authority: ctx.accounts.wallet,
                token_program: ctx.accounts.token_program,
            },
            &[],
        )?;
    }

    Ok(())
}
