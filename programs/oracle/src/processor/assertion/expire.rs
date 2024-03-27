use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::{Context, ExpireAssertionAccounts};
use crate::instruction::ExpireAssertionArgs;
use crate::pda;
use crate::state::{AccountSized, Assertion, Request, RequestState};

pub fn expire<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: ExpireAssertionArgs,
) -> ProgramResult {
    let ctx = ExpireAssertionAccounts::context(accounts)?;

    match args {
        ExpireAssertionArgs::V1 { .. } => expire_v1(program_id, ctx, args),
    }
}

fn expire_v1(
    _program_id: &Pubkey,
    ctx: Context<ExpireAssertionAccounts>,
    args: ExpireAssertionArgs,
) -> ProgramResult {
    let ExpireAssertionArgs::V1 {} = args;

    let ExpireAssertionAccounts { request: request_info, assertion } = ctx.accounts;

    let mut request = Request::from_account_info_mut(request_info)?;

    request.assert_pda(request_info.key)?;

    pda::assertion::assert_pda(assertion.key, request_info.key)?;

    match request.state {
        RequestState::Asserted => {}
        RequestState::Requested => return Err(OracleError::NotAsserted.into()),
        RequestState::Disputed => return Err(OracleError::AlreadyDisputed.into()),
        RequestState::Resolved => return Err(OracleError::AlreadyResolved.into()),
    }

    let mut assertion = Assertion::from_account_info_mut(assertion)?;

    let now = Clock::get()?;

    if now.unix_timestamp < assertion.expiration_timestamp {
        return Err(OracleError::DisputeExpireTooEarly.into());
    }

    let value = assertion.asserted_value;

    request.value = value;
    request.resolve_timestamp = now.unix_timestamp;
    request.state = RequestState::Resolved;
    request.save()?;

    assertion.resolved_value = value;
    assertion.save()?;

    // TODO: Emit an event?

    Ok(())
}
