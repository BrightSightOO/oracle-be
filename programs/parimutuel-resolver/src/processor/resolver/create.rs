use oracle::state::RequestData;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::ResolverError;
use crate::instruction::accounts::{Context, CreateResolverAccounts};
use crate::instruction::CreateResolverArgs;
use crate::state::{InitAccount, InitContext, InitResolver, Resolver};
use crate::{cpi, pda, utils};

pub fn create<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateResolverArgs,
) -> ProgramResult {
    let ctx = CreateResolverAccounts::context(accounts)?;

    match args {
        CreateResolverArgs::V1 { .. } => create_v1(program_id, ctx, args),
    }
}

fn create_v1(
    program_id: &Pubkey,
    ctx: Context<CreateResolverAccounts>,
    args: CreateResolverArgs,
) -> ProgramResult {
    let CreateResolverArgs::V1 {} = args;

    let CreateResolverAccounts { resolver, market, request, payer, system_program } = ctx.accounts;

    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    utils::assert_system_program(system_program.key)?;

    let resolver_bump = pda::resolver::assert_pda(resolver.key, market.key)?;

    // Step 1: Check `request` is a yes/no request.
    {
        let request = cpi::oracle::request_from_account_info(request)?;

        if !matches!(request.data, RequestData::YesNo { .. }) {
            return Err(ResolverError::InvalidRequest.into());
        }
    }

    // Step 2: Initialize `resolver` account.
    {
        let signer_seeds = pda::resolver::seeds_with_bump(market.key, &resolver_bump);

        Resolver::init(InitResolver { market: *market.key, request: *request.key }).save(
            InitContext {
                account: resolver,
                payer,
                system_program,
                program_id,
                signer_seeds: &[&signer_seeds],
            },
        )?;
    }

    Ok(())
}
