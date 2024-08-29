use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::CloseVotingV1Accounts;
use crate::pda;
use crate::state::{Account, AccountSized, ConfigV1, RequestState, RequestV1, VotingV1};

pub fn close_voting_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = CloseVotingV1Accounts::context(accounts)?;

    let voting_window: u32;
    let arbitration_window: u32;

    // Step 1: Get config voting and arbitration windows.
    {
        let config = ConfigV1::from_account_info(ctx.accounts.config)?;

        voting_window = config.voting_window;
        arbitration_window = config.arbitration_window;
    }

    let mut request = RequestV1::from_account_info_mut(ctx.accounts.request)?;

    // Step 2: Check voting has not yet resolved the request.
    {
        // Guard request.
        request.assert_pda(ctx.accounts.request.key)?;
        request.assert_config(ctx.accounts.config.key)?;

        // If the request state is not `Disputed`,
        // then the voting must have ended and resolved the request.
        if request.state != RequestState::Disputed {
            return Err(OracleError::NotDisputed.into());
        }
    }

    // Guard voting PDA.
    pda::voting::assert_pda(ctx.accounts.voting.key, ctx.accounts.request.key)?;

    let now = Clock::get()?.unix_timestamp;

    let mut voting = VotingV1::from_account_info_mut(ctx.accounts.voting)?;

    // Step 3: Check the voting window has expired.
    if now < voting.end_timestamp {
        return Err(OracleError::VotingWindowNotExpired.into());
    }

    // Step 4: If the request has an arbitrator, check the arbitration window has expired.
    if request.has_arbitrator() && arbitration_window > 0 {
        let end_timestamp = checked_add!(voting.end_timestamp, i64::from(arbitration_window))?;

        if now < end_timestamp {
            return Err(OracleError::ArbitrationWindowNotExpired.into());
        }
    }

    // Step 5: If no votes were cast then start a new vote window.
    if voting.vote_count == 0 {
        log!("Not enough votes cast - starting new vote window");

        voting.start_timestamp = now;
        voting.end_timestamp = checked_add!(now, i64::from(voting_window))?;

        voting.save()?;

        // TODO: Emit an event?

        return Ok(());
    }

    // Voting account is not mutated when resolving.
    let voting = voting.into_inner();

    // Step 6: Resolve the request with the modal voted value.
    {
        // Update request with resolved value.
        request.resolve_timestamp = now;
        request.state = RequestState::Resolved;
        request.value = voting.mode_value;

        request.save()?;
    }

    // TODO: Emit an event?

    Ok(())
}
