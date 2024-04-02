use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::cpi::spl::{CreateTokenAccount, TransferChecked};
use crate::error::OracleError;
use crate::instruction::accounts::{Context, DisputeAssertionAccounts};
use crate::instruction::DisputeAssertionArgs;
use crate::state::{AccountSized, Assertion, Request, RequestState};
use crate::{cpi, pda, utils};

pub fn dispute<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: DisputeAssertionArgs,
) -> ProgramResult {
    let ctx = DisputeAssertionAccounts::context(accounts)?;

    match args {
        DisputeAssertionArgs::V1 { .. } => dispute_v1(program_id, ctx, args),
    }
}

fn dispute_v1(
    _program_id: &Pubkey,
    ctx: Context<DisputeAssertionAccounts>,
    args: DisputeAssertionArgs,
) -> ProgramResult {
    let DisputeAssertionArgs::V1 { value } = args;

    let DisputeAssertionAccounts {
        request,
        assertion,
        bond_mint,
        bond_source,
        bond_escrow,
        disputer,
        payer,
        token_program,
        system_program,
    } = ctx.accounts;

    if !disputer.is_signer || !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    utils::assert_token_program(token_program.key)?;
    utils::assert_system_program(system_program.key)?;

    let bond: u64;

    let now = Clock::get()?;

    // Step 1: Update request and assertion states.
    {
        let request_address = request.key;

        let mut request = Request::from_account_info_mut(request)?;

        // Check request state.
        {
            request.assert_pda(request_address)?;

            match request.state {
                RequestState::Asserted => {}
                RequestState::Requested => return Err(OracleError::NotAsserted.into()),
                RequestState::Disputed => return Err(OracleError::AlreadyDisputed.into()),
                RequestState::Resolved => return Err(OracleError::AlreadyResolved.into()),
            }
        }

        // Check assertion PDA.
        pda::assertion::assert_pda(assertion.key, request_address)?;

        // Check bond mint address matches.
        request.validate_bond_mint(bond_mint.key)?;

        bond = request.bond;

        let mut assertion = Assertion::from_account_info_mut(assertion)?;

        // Check the disputer differs from the asserter.
        if !common::cmp_pubkeys(&assertion.asserter, disputer.key) {
            return Err(OracleError::DisputerIsAsserter.into());
        }

        // Check the dispute window has not expired.
        assertion.validate_dispute_timestamp(now.unix_timestamp)?;

        // Validate disputed value and check it does not fall within acceptable deviation.
        request.data.validate_value(value)?;
        request.data.validate_dispute(assertion.asserted_value, value)?;

        // Step 1.1: Update request state.
        {
            request.state = RequestState::Disputed;

            request.save()?;
        }

        // Step 1.2: Update assertion state.
        {
            assertion.disputer = *disputer.key;
            assertion.disputed_value = value;

            assertion.save()?;
        }
    }

    // Step 2: Transfer bond to escrow.
    {
        let mint_decimals = cpi::spl::get_decimals(bond_mint)?;

        // Step 2.1: Initialize `bond_escrow` account.
        {
            let bond_bump = pda::dispute_bond::assert_pda(bond_escrow.key, request.key)?;
            let signer_seeds = pda::dispute_bond::seeds_with_bump(request.key, &bond_bump);

            cpi::spl::create_token_account(
                request.key,
                CreateTokenAccount {
                    account: bond_escrow,
                    mint: bond_mint,
                    payer,
                    token_program,
                    system_program,
                },
                &[&signer_seeds],
            )?;
        }

        // Step 2.2: Transfer bond from `bond_source` to `bond_escrow`.
        cpi::spl::transfer_checked(
            bond,
            mint_decimals,
            TransferChecked {
                source: bond_source,
                destination: bond_escrow,
                mint: bond_mint,
                authority: disputer,
                token_program,
            },
            &[],
        )?;
    }

    // FIXME: Initialize dispute account for voting.

    // TODO: Emit an event?

    Ok(())
}
