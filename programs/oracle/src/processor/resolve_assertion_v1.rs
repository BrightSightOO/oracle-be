use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::OracleError;
use crate::instruction::accounts::ResolveAssertionV1Accounts;
use crate::pda;
use crate::state::{Account, AccountSized, AssertionV1, RequestState, RequestV1};

pub fn resolve_assertion_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = ResolveAssertionV1Accounts::context(accounts)?;

    let mut request = RequestV1::from_account_info_mut(ctx.accounts.request)?;

    // Guard request PDA.
    request.assert_pda(ctx.accounts.request.key)?;

    // The request state must be `Asserted` to resolve.
    match request.state {
        RequestState::Asserted => {}
        RequestState::Requested => return Err(OracleError::NotAsserted.into()),
        RequestState::Disputed => return Err(OracleError::AlreadyDisputed.into()),
        RequestState::Resolved => return Err(OracleError::AlreadyResolved.into()),
    }

    // Guard assertion PDA.
    pda::assertion::assert_pda(ctx.accounts.assertion.key, ctx.accounts.request.key)?;

    let assertion = AssertionV1::from_account_info(ctx.accounts.assertion)?;

    let now = Clock::get()?.unix_timestamp;

    // An assertion can only be resolved if it reaches the expiration timestamp undisputed.
    assertion.validate_expiration_timestamp(now)?;

    request.resolve_timestamp = now;
    request.state = RequestState::Resolved;
    request.value = assertion.asserted_value;

    request.save()?;

    // TODO: Emit an event?

    Ok(())
}
