use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;
use crate::instruction::accounts::ClaimVoteV1Accounts;
use crate::state::{Account, AssertionV1, RequestState, RequestV1, StakeV1, VoteV1, VotingV1};
use crate::{pda, utils};

pub fn claim_vote_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = ClaimVoteV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.voter)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Step 1: Check stake voter.
    {
        let stake = StakeV1::from_account_info(ctx.accounts.stake)?;

        // Guard stake.
        stake.assert_voter(ctx.accounts.voter.key)?;
    }

    let request_index: u64;
    let request_bump: u8;

    let resolved_value: u64;
    let bond: u64;

    // Step 2: Check request state.
    {
        let request = RequestV1::from_account_info(ctx.accounts.request)?;

        // Guard request.
        request_bump = request.assert_pda(ctx.accounts.request.key)?;
        request.assert_bond_mint(ctx.accounts.bond_mint.key)?;

        // The request must be resolved to claim.
        if request.state != RequestState::Resolved {
            return Err(OracleError::NotResolved.into());
        }

        request_index = request.index;
        resolved_value = request.value;
        bond = request.bond;
    }

    // Guard PDAs.
    pda::assertion::assert_pda(ctx.accounts.assertion.key, ctx.accounts.request.key)?;
    pda::voting::assert_pda(ctx.accounts.voting.key, ctx.accounts.request.key)?;
    pda::vote::assert_pda(ctx.accounts.vote.key, ctx.accounts.voting.key, ctx.accounts.stake.key)?;

    let votes: u64;

    // Step 3: Get voter votes for resolved value.
    {
        let vote = VoteV1::from_account_info(ctx.accounts.vote)?;

        // The vote must be for the resolved value.
        if vote.value != resolved_value {
            return Err(OracleError::IncorrectVote.into());
        }

        votes = vote.votes;
    }

    let total_votes: u64;

    // Step 4: Get total votes for resolved value.
    {
        let voting = VotingV1::from_account_info(ctx.accounts.request)?;

        total_votes = match voting.votes.get(&resolved_value) {
            // The resolved value matches the voted value; any value voted for should have an entry
            // with at least one vote in the votes map. Thus this branch should be unreachable.
            None | Some(0) => unreachable!("total votes should not be zero"),
            Some(total_votes) => *total_votes,
        };
    }

    let voter_reward = (((bond as u128) * (votes as u128)) / (total_votes as u128)) as u64;

    log!("Votes: {votes} / {total_votes}");
    log!("Reward: {voter_reward}");

    // Step 5: Check bond escrow for incorrect asserter/disputer.
    {
        let assertion = AssertionV1::from_account_info(ctx.accounts.assertion)?;

        // Check if the asserted value matches the resolved value.
        if resolved_value == assertion.asserted_value {
            log!("Assertion is correct");

            // The resolved value matches the asserted value, so the disputer loses their bond.
            pda::dispute_bond::assert_pda(ctx.accounts.bond_escrow.key, ctx.accounts.request.key)?;
        } else {
            log!("Assertion is incorrect");

            // The resolved value doesn't match the asserted value, so the asserter loses their bond.
            pda::assert_bond::assert_pda(ctx.accounts.bond_escrow.key, ctx.accounts.request.key)?;
        }
    }

    // Step 6: Claim voter reward from incorrect bond.
    {
        let signer_seeds = pda::request::seeds_with_bump(&request_index, &request_bump);

        let decimals = cpi::spl::mint_decimals(ctx.accounts.bond_mint)?;

        cpi::spl::transfer_checked(
            voter_reward,
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
    }

    // Step 7: Close vote account.
    common::close_account(ctx.accounts.vote, ctx.accounts.voter)?;

    Ok(())
}
