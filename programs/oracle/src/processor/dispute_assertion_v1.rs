use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::DisputeAssertionV1Accounts;
use crate::state::{
    Account, AccountSized, AssertionV1, ConfigV1, InitAccount, InitContext, InitVoting,
    RequestState, RequestV1, VotingV1,
};
use crate::{pda, utils};

pub fn dispute_assertion_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = DisputeAssertionV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.disputer)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    let voting_window: u32;

    // Step 1: Get config voting window.
    {
        let config = ConfigV1::from_account_info(ctx.accounts.config)?;

        voting_window = config.voting_window;
    }

    let now = Clock::get()?.unix_timestamp;
    let bond: u64;

    // Step 2: Update request and assertion states.
    {
        let mut request = RequestV1::from_account_info_mut(ctx.accounts.request)?;

        // Step 2.1: Check request.
        {
            // Guard request.
            request.assert_pda(ctx.accounts.request.key)?;
            request.assert_config(ctx.accounts.config.key)?;
            request.assert_bond_mint(ctx.accounts.bond_mint.key)?;

            // The request state must be `Asserted` to dispute.
            match request.state {
                RequestState::Asserted => {}
                RequestState::Requested => return Err(OracleError::NotAsserted.into()),
                RequestState::Disputed => return Err(OracleError::AlreadyDisputed.into()),
                RequestState::Resolved => return Err(OracleError::AlreadyResolved.into()),
            }
        }

        bond = request.bond;

        // Step 2.2: Check and update assertion.
        {
            // Guard assertion PDA.
            pda::assertion::assert_pda(ctx.accounts.assertion.key, ctx.accounts.request.key)?;

            let mut assertion = AssertionV1::from_account_info_mut(ctx.accounts.assertion)?;

            // The disputer cannot have the same address as the asserter.
            if common::cmp_pubkeys(&assertion.asserter, ctx.accounts.disputer.key) {
                return Err(OracleError::DisputerIsAsserter.into());
            }

            // The dispute window of the assertion must not have expired.
            assertion.validate_dispute_timestamp(now)?;

            assertion.disputer = *ctx.accounts.disputer.key;
            assertion.save()?;
        }

        // Step 2.3: Update request state.
        {
            request.state = RequestState::Disputed;
            request.save()?;
        }
    }

    // Step 3: Transfer bond to escrow.
    {
        let mint_decimals = cpi::spl::mint_decimals(ctx.accounts.bond_mint)?;

        // Step 3.1: Initialize `bond_escrow` account.
        {
            let bump = pda::dispute_bond::assert_pda(
                ctx.accounts.bond_escrow.key,
                ctx.accounts.request.key,
            )?;
            let signer_seeds = pda::dispute_bond::seeds_with_bump(ctx.accounts.request.key, &bump);

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
                authority: ctx.accounts.disputer,
                token_program: ctx.accounts.token_program,
            },
            &[],
        )?;
    }

    // Step 4: Initialize `voting` account.
    {
        let bump = pda::voting::assert_pda(ctx.accounts.voting.key, ctx.accounts.request.key)?;
        let signer_seeds = pda::voting::seeds_with_bump(ctx.accounts.request.key, &bump);

        VotingV1::try_init(InitVoting {
            request: *ctx.accounts.request.key,
            start_timestamp: now,
            voting_window,
        })?
        .save(InitContext {
            account: ctx.accounts.voting,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    // TODO: Emit an event?

    Ok(())
}
