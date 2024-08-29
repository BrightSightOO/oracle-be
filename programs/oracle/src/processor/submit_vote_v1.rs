use std::collections::btree_map::Entry;

use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::SubmitVoteV1Accounts;
use crate::state::{
    Account, AccountSized, ConfigV1, InitAccount, InitContext, InitVote, RequestState, RequestV1,
    StakeV1, VoteV1, VotingV1,
};
use crate::{pda, utils};

#[derive(Clone, BorshDeserialize)]
pub struct SubmitVoteV1Args {
    /// Value to vote for.
    pub value: u64,
}

pub fn submit_vote_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: SubmitVoteV1Args,
) -> ProgramResult {
    let ctx = SubmitVoteV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.voter)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    let voting_window: u32;

    // Step 1: Get config voting window.
    {
        let config = ConfigV1::from_account_info(ctx.accounts.config)?;

        voting_window = config.voting_window;
    }

    // Step 2: Check voting has not yet resolved the request.
    {
        let request = RequestV1::from_account_info(ctx.accounts.request)?;

        // Guard request.
        request.assert_pda(ctx.accounts.request.key)?;
        request.assert_config(ctx.accounts.config.key)?;

        // If the request state is not `Disputed`,
        // then the voting must have ended and resolved the request.
        if request.state != RequestState::Disputed {
            return Err(OracleError::NotDisputed.into());
        }
    }

    // Guard PDAs.
    pda::voting::assert_pda(ctx.accounts.voting.key, ctx.accounts.request.key)?;

    let now = Clock::get()?.unix_timestamp;

    let mut voting = VotingV1::from_account_info_mut(ctx.accounts.voting)?;

    // Step 3: Check the voting window hasn't expired.
    if voting.end_timestamp <= now {
        // TODO: We should probably require a minimum number of votes.

        // If the vote count is non-zero, then the voting window is considered expired.
        if voting.vote_count != 0 {
            return Err(OracleError::VotingWindowExpired.into());
        }

        // If no votes were cast then start a new vote window.
        log!("Not enough votes cast - starting new vote window");

        voting.start_timestamp = now;
        voting.end_timestamp = checked_add!(now, i64::from(voting_window))?;
    }

    let votes: u64;

    // Step 4: Get stake votes and update lock.
    {
        let mut stake = StakeV1::from_account_info_mut(ctx.accounts.stake)?;

        // Guard stake voter.
        stake.assert_voter(ctx.accounts.voter.key)?;

        // Lock the stake until the end of the vote window.
        stake.lock_timestamp = voting.end_timestamp;

        votes = stake.amount;
    }

    // Step 5: Initialize `vote` account.
    {
        let bump = pda::vote::assert_pda(
            ctx.accounts.vote.key,
            ctx.accounts.voting.key,
            ctx.accounts.stake.key,
        )?;
        let signer_seeds =
            pda::vote::seeds_with_bump(ctx.accounts.voting.key, ctx.accounts.stake.key, &bump);

        VoteV1::init(InitVote { stake: *ctx.accounts.stake.key, value: args.value, votes }).save(
            InitContext {
                account: ctx.accounts.vote,
                payer: ctx.accounts.payer,
                system_program: ctx.accounts.system_program,
                program_id,
                signers_seeds: &[&signer_seeds],
            },
        )?;
    }

    // Step 6: Add votes for the submitted value.
    {
        // Add submitted votes for the voted value, the get the new amount of votes for that value.
        let freq = match voting.votes.entry(args.value) {
            // An entry exists for the voted value. Add the submitted votes to the entry.
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                let freq = checked_add!(entry, votes)?;

                *entry = freq;

                freq
            }
            // No entry exists for the voted value. Create a new entry with the submitted votes.
            Entry::Vacant(entry) => {
                entry.insert(votes);

                votes
            }
        };

        voting.vote_count = checked_add!(voting.vote_count, votes)?;

        // If the frequency of the value voted for is greater than the frequency of
        // the previous modal value, then update the modal value.
        if freq > voting.votes.get(&voting.mode_value).copied().unwrap_or_default() {
            voting.mode_value = args.value;
        }
    }

    voting.realloc(ctx.accounts.payer, ctx.accounts.system_program)?;
    voting.save()?;

    // TODO: Emit an event?

    Ok(())
}
