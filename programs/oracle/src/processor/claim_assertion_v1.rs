use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;
use crate::instruction::accounts::ClaimAssertionV1Accounts;
use crate::state::{Account, AssertionV1, RequestState, RequestV1};
use crate::{pda, utils};

pub fn claim_assertion_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = ClaimAssertionV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.asserter)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    let request_index: u64;
    let request_bump: u8;

    {
        let resolved_value: u64;

        // Step 1: Check request state.
        {
            let request = RequestV1::from_account_info(ctx.accounts.request)?;

            // Guard request.
            request_bump = request.assert_pda(ctx.accounts.request.key)?;
            request.assert_reward_mint(ctx.accounts.reward_mint.key)?;
            request.assert_bond_mint(ctx.accounts.bond_mint.key)?;

            // The request must be resolved to claim.
            if request.state != RequestState::Resolved {
                return Err(OracleError::NotResolved.into());
            }

            request_index = request.index;
            resolved_value = request.value;
        }

        // Step 2: Check assertion.
        {
            // Guard assertion PDA.
            pda::assertion::assert_pda(ctx.accounts.assertion.key, ctx.accounts.request.key)?;

            let assertion = AssertionV1::from_account_info(ctx.accounts.assertion)?;

            // Guard assertion.
            assertion.assert_asserter(ctx.accounts.asserter.key)?;

            // The asserter can only claim if the asserted value is correct.
            if assertion.asserted_value != resolved_value {
                return Err(OracleError::IncorrectClaimer.into());
            }
        }
    }

    let signer_seeds = pda::request::seeds_with_bump(&request_index, &request_bump);

    // Step 3: Recover asserter bond.
    {
        pda::assert_bond::assert_pda(ctx.accounts.bond_escrow.key, ctx.accounts.request.key)?;

        let bond = cpi::spl::account_amount(ctx.accounts.bond_escrow)?;
        let decimals = cpi::spl::mint_decimals(ctx.accounts.bond_mint)?;

        // Step 3.1: Transfer bond from escrow to asserter.
        cpi::spl::transfer_checked(
            bond,
            decimals,
            cpi::spl::TransferChecked {
                source: ctx.accounts.bond_escrow,
                destination: ctx.accounts.bond_destination,
                mint: ctx.accounts.bond_mint,
                authority: ctx.accounts.request,
                token_program: ctx.accounts.token_program,
            },
            &[&signer_seeds],
        )?;

        // Step 3.2: Close bond escrow account.
        cpi::spl::close_account(
            cpi::spl::CloseAccount {
                account: ctx.accounts.bond_escrow,
                destination: ctx.accounts.asserter,
                authority: ctx.accounts.request,
                token_program: ctx.accounts.token_program,
            },
            &[&signer_seeds],
        )?;
    }

    // Step 4: Claim reward.
    {
        pda::reward::assert_pda(ctx.accounts.reward_escrow.key, ctx.accounts.request.key)?;

        let reward = cpi::spl::account_amount(ctx.accounts.reward_escrow)?;
        let decimals = cpi::spl::mint_decimals(ctx.accounts.reward_mint)?;

        // Step 4.1: Transfer reward from escrow to asserter.
        cpi::spl::transfer_checked(
            reward,
            decimals,
            cpi::spl::TransferChecked {
                source: ctx.accounts.reward_escrow,
                destination: ctx.accounts.reward_destination,
                mint: ctx.accounts.reward_mint,
                authority: ctx.accounts.request,
                token_program: ctx.accounts.token_program,
            },
            &[&signer_seeds],
        )?;

        // Step 4.2: Close reward escrow account.
        cpi::spl::close_account(
            cpi::spl::CloseAccount {
                account: ctx.accounts.reward_escrow,
                destination: ctx.accounts.asserter,
                authority: ctx.accounts.request,
                token_program: ctx.accounts.token_program,
            },
            &[&signer_seeds],
        )?;
    }

    Ok(())
}
