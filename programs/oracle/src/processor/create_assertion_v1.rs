use borsh::BorshDeserialize;
use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::CreateAssertionV1Accounts;
use crate::state::{
    Account, AccountSized, AssertionV1, ConfigV1, InitAccount, InitAssertion, InitContext,
    RequestState, RequestV1,
};
use crate::{pda, utils};

#[derive(Clone, BorshDeserialize)]
pub struct CreateAssertionV1Args {
    /// Value to assert.
    pub value: u64,
}

pub fn create_assertion_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateAssertionV1Args,
) -> ProgramResult {
    let ctx = CreateAssertionV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.asserter)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    let dispute_window: u32;

    // Step 1: Get config dispute window.
    {
        let config = ConfigV1::from_account_info(ctx.accounts.config)?;

        dispute_window = config.dispute_window;
    }

    let bond: u64;
    let now = Clock::get()?.unix_timestamp;

    // Step 2: Update request state.
    {
        let mut request = RequestV1::from_account_info_mut(ctx.accounts.request)?;

        // Guard request.
        request.assert_pda(ctx.accounts.request.key)?;
        request.assert_config(ctx.accounts.config.key)?;
        request.assert_bond_mint(ctx.accounts.bond_mint.key)?;

        // If the request state is not `Requested`, then an assertion has already been made.
        if request.state != RequestState::Requested {
            return Err(OracleError::AlreadyAsserted.into());
        }

        // The assertion timestamp on the request must have been reached.
        request.validate_assertion_timestamp(now)?;
        // The asserted value must be valid for the request data type.
        request.data.validate_value(args.value)?;

        bond = request.bond;

        request.state = RequestState::Asserted;
        request.save()?;
    }

    // Step 3: Initialize `assertion` account.
    {
        let bump =
            pda::assertion::assert_pda(ctx.accounts.assertion.key, ctx.accounts.request.key)?;
        let signer_seeds = pda::assertion::seeds_with_bump(ctx.accounts.request.key, &bump);

        AssertionV1::try_init(InitAssertion {
            request: *ctx.accounts.request.key,
            assertion_timestamp: now,
            asserter: *ctx.accounts.asserter.key,
            asserted_value: args.value,
            dispute_window,
        })?
        .save(InitContext {
            account: ctx.accounts.assertion,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    // Step 3: Transfer bond to escrow.
    {
        let mint_decimals = cpi::spl::mint_decimals(ctx.accounts.bond_mint)?;

        // Step 3.1: Initialize `bond_escrow` account.
        {
            let bond_bump = pda::assert_bond::assert_pda(
                ctx.accounts.bond_escrow.key,
                ctx.accounts.request.key,
            )?;
            let signer_seeds =
                pda::assert_bond::seeds_with_bump(ctx.accounts.request.key, &bond_bump);

            cpi::spl::create_token_account(
                ctx.accounts.request.key,
                cpi::spl::CreateTokenAccount {
                    account: ctx.accounts.bond_escrow,
                    mint: ctx.accounts.bond_mint,
                    payer: ctx.accounts.payer,
                    token_program: ctx.accounts.token_program,
                    system_program: ctx.accounts.system_program,
                },
                &[&signer_seeds],
            )?;
        }

        // Step 3.2: Transfer bond from `bond_source` to `bond_escrow`.
        cpi::spl::transfer_checked(
            bond,
            mint_decimals,
            cpi::spl::TransferChecked {
                source: ctx.accounts.bond_source,
                destination: ctx.accounts.bond_escrow,
                mint: ctx.accounts.bond_mint,
                authority: ctx.accounts.asserter,
                token_program: ctx.accounts.token_program,
            },
            &[],
        )?;
    }

    // TODO: Emit an event?

    Ok(())
}
