use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::cpi::spl::{CreateTokenAccount, TransferChecked};
use crate::error::OracleError;
use crate::instruction::accounts::{Context, CreateAssertionAccounts};
use crate::instruction::CreateAssertionArgs;
use crate::state::{
    AccountSized, Assertion, InitAccount, InitAssertion, InitContext, Request, RequestState,
};
use crate::{cpi, pda, utils};

pub fn create<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateAssertionArgs,
) -> ProgramResult {
    let ctx = CreateAssertionAccounts::context(accounts)?;

    match args {
        CreateAssertionArgs::V1 { .. } => create_v1(program_id, ctx, args),
    }
}

fn create_v1(
    program_id: &Pubkey,
    ctx: Context<CreateAssertionAccounts>,
    args: CreateAssertionArgs,
) -> ProgramResult {
    let CreateAssertionArgs::V1 { bond, value } = args;

    let CreateAssertionAccounts {
        oracle,
        request,
        assertion,
        bond_mint,
        bond_source,
        bond_escrow,
        governance_mint,
        governance_source,
        governance_escrow,
        asserter,
        payer,
        token_program,
        system_program,
    } = ctx.accounts;

    if !asserter.is_signer || !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    utils::assert_token_program(token_program.key)?;
    utils::assert_system_program(system_program.key)?;

    pda::oracle::assert_pda(oracle.key)?;

    if bond < crate::MIN_BOND {
        return Err(OracleError::InsufficientBond.into());
    }

    // TODO: Use marker accounts to check valid bond mints.
    // TODO: Check governance mint address.

    // Step 1: Update request state.
    {
        let request_address = request.key;

        let mut request = Request::from_account_info_mut(request)?;

        request.assert_pda(request_address)?;
        request.assert_requested()?;

        request.state = RequestState::Asserted;
        request.save()?;
    }

    // Step 2: Initialize `assertion` account.
    {
        let assertion_bump = pda::assertion::assert_pda(assertion.key, request.key)?;
        let signer_seeds = pda::assertion::seeds_with_bump(request.key, &assertion_bump);

        Assertion::try_init(InitAssertion {
            request: *request.key,
            governance: crate::GOVERNANCE_BOND,
            bond,
            bond_mint: *bond_mint.key,
            assertion_timestamp: Clock::get()?.unix_timestamp,
            asserter: *asserter.key,
            asserted_value: value,
        })?
        .save(InitContext {
            account: request,
            payer,
            system_program,
            program_id,
            signer_seeds: &[&signer_seeds],
        })?;
    }

    // Step 3: Transfer bond to escrow.
    {
        let mint_decimals = cpi::spl::get_decimals(bond_mint)?;

        // Step 3.1: Initialize `bond_escrow` account.
        {
            let bond_bump = pda::assert_bond::assert_pda(bond_escrow.key, request.key)?;
            let signer_seeds = pda::assert_bond::seeds_with_bump(request.key, &bond_bump);

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

        // Step 3.2: Transfer bond from `bond_source` to `bond_escrow`.
        cpi::spl::transfer_checked(
            bond,
            mint_decimals,
            TransferChecked {
                source: bond_source,
                destination: bond_escrow,
                mint: bond_mint,
                authority: asserter,
                token_program,
            },
            &[],
        )?;
    }

    // Step 4: Transfer governance to escrow.
    {
        let mint_decimals = cpi::spl::get_decimals(governance_mint)?;

        // Step 3.1: Initialize `governance_escrow` account.
        {
            let governance_bump =
                pda::assert_governance::assert_pda(governance_escrow.key, request.key)?;
            let signer_seeds =
                pda::assert_governance::seeds_with_bump(request.key, &governance_bump);

            cpi::spl::create_token_account(
                request.key,
                CreateTokenAccount {
                    account: governance_escrow,
                    mint: governance_mint,
                    payer,
                    token_program,
                    system_program,
                },
                &[&signer_seeds],
            )?;
        }

        // Step 3.2: Transfer governance from `governance_source` to `governance_escrow`.
        cpi::spl::transfer_checked(
            crate::GOVERNANCE_BOND,
            mint_decimals,
            TransferChecked {
                source: governance_source,
                destination: governance_escrow,
                mint: governance_mint,
                authority: asserter,
                token_program,
            },
            &[],
        )?;
    }

    // TODO: Emit an event for the request?

    Ok(())
}
