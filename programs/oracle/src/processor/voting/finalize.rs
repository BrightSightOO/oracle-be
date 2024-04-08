use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::{Context, FinalizeVotingAccounts};
use crate::instruction::FinalizeVotingArgs;
use crate::pda;
use crate::state::{AccountSized, Request, RequestState, Voting};

pub fn finalize<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: FinalizeVotingArgs,
) -> ProgramResult {
    let ctx = FinalizeVotingAccounts::context(accounts)?;

    match args {
        FinalizeVotingArgs::V1 { .. } => finalize_v1(program_id, ctx, args),
    }
}

fn finalize_v1(
    _program_id: &Pubkey,
    ctx: Context<FinalizeVotingAccounts>,
    args: FinalizeVotingArgs,
) -> ProgramResult {
    let FinalizeVotingArgs::V1 {} = args;

    let FinalizeVotingAccounts { request, voting } = ctx.accounts;

    let request_address = request.key;

    let mut request = Request::from_account_info_mut(request)?;

    // Step 1: Check request.
    {
        request.assert_pda(request_address)?;

        if request.state != RequestState::Disputed {
            return Err(OracleError::NotDisputed.into());
        }
    }

    pda::voting::assert_pda(voting.key, request_address)?;

    let now = Clock::get()?;

    let mut voting = Voting::from_account_info_mut(voting)?;

    // Step 2: Check the voting window has expired.
    if now.unix_timestamp < voting.end_timestamp {
        return Err(OracleError::VotingWindowNotExpired.into());
    }

    // If no votes were cast then start a new vote window.
    if voting.vote_count == 0 {
        log!("No votes cast - starting new vote window");

        voting.start_timestamp = now.unix_timestamp;
        voting.end_timestamp = increment!(now.unix_timestamp, crate::VOTING_WINDOW)?;

        voting.save()?;

        // TODO: Emit an event?

        return Ok(());
    }

    // Voting account is not mutated when resolving.
    let voting = voting.into_inner();

    // Step 3: Update request with resolved value.
    request.resolve_timestamp = now.unix_timestamp;
    request.state = RequestState::Resolved;
    request.value = voting.mode_value;

    // TODO: Emit an event?

    Ok(())
}
