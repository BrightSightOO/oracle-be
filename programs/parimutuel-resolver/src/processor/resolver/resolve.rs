use oracle::state::RequestState;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::cpi::parimutuel::{Outcome, UpdateState};
use crate::error::ResolverError;
use crate::instruction::accounts::{Context, ResolveAccounts};
use crate::instruction::ResolveArgs;
use crate::state::{Account, Resolver};
use crate::{cpi, pda, utils};

pub fn resolve<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: ResolveArgs,
) -> ProgramResult {
    let ctx = ResolveAccounts::context(accounts)?;

    match args {
        ResolveArgs::V1 { .. } => resolve_v1(program_id, ctx, args),
    }
}

fn resolve_v1(
    _program_id: &Pubkey,
    ctx: Context<ResolveAccounts>,
    args: ResolveArgs,
) -> ProgramResult {
    let ResolveArgs::V1 {} = args;

    let ResolveAccounts { resolver, market, request, parimutuel_program } = ctx.accounts;

    utils::assert_parimutuel_program(parimutuel_program.key)?;

    let resolver_bump = pda::resolver::assert_pda(resolver.key, market.key)?;

    // Step 1: Check request address matches.
    {
        let resolver = Resolver::from_account_info(resolver)?;

        if !common::cmp_pubkeys(request.key, &resolver.request) {
            return Err(ResolverError::IncorrectRequest.into());
        }
    }

    let request = cpi::oracle::request_from_account_info(request)?;

    if request.state != RequestState::Resolved {
        return Err(ResolverError::NotResolved.into());
    }

    let outcome = match request.value {
        0 => Outcome::No,
        1 => Outcome::Yes,
        _ => Outcome::Invalid,
    };

    log!("Resolved request value: {}", request.value);
    log!("Outcome: {outcome:?}");

    // Step 2: Update market state.
    {
        let signer_seeds = pda::resolver::seeds_with_bump(market.key, &resolver_bump);

        cpi::parimutuel::update_state(
            outcome,
            UpdateState { market, resolver, parimutuel_program },
            &[&signer_seeds],
        )?;
    }

    Ok(())
}
