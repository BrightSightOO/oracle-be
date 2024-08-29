use borsh::BorshDeserialize;
use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::clock::UnixTimestamp;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;
use crate::instruction::accounts::CreateRequestV1Accounts;
use crate::state::{
    Account, AccountSized, ConfigV1, CurrencyV1, InitAccount, InitContext, InitRequest, OracleV1,
    RequestData, RequestV1,
};
use crate::{pda, utils};

#[derive(Clone, BorshDeserialize)]
pub struct CreateRequestV1Args {
    /// Amount rewarded to the asserter/disputer on resolution.
    pub reward: u64,
    /// Amount to required to bond in order to assert/dispute value.
    pub bond: u64,
    /// Unix timestamp after which a value can be asserted.
    pub timestamp: UnixTimestamp,
    /// Arbitrator address.
    pub arbitrator: Pubkey,
    /// Request data.
    pub data: RequestData,
}

pub fn create_request_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateRequestV1Args,
) -> ProgramResult {
    let ctx = CreateRequestV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.creator)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Guard PDAs.
    pda::oracle::assert_pda(ctx.accounts.oracle.key)?;

    // Step 1: Check reward.
    {
        let reward_currency = CurrencyV1::from_account_info(ctx.accounts.reward_currency)?;

        // Guard currency.
        reward_currency.assert_pda(ctx.accounts.reward_currency.key)?;
        reward_currency.assert_config(ctx.accounts.config.key)?;
        reward_currency.assert_mint(ctx.accounts.reward_mint.key)?;

        // Check the reward bounds.
        if !reward_currency.reward_range.contains(args.reward) {
            return Err(OracleError::RewardBounds.into());
        }
    }

    let bond_mint: Pubkey;

    // Step 2: Check bond.
    {
        let bond_currency = CurrencyV1::from_account_info(ctx.accounts.bond_currency)?;

        // Guard currency.
        bond_currency.assert_pda(ctx.accounts.bond_currency.key)?;
        bond_currency.assert_config(ctx.accounts.config.key)?;

        // Check the bond bounds.
        if !bond_currency.bond_range.contains(args.bond) {
            return Err(OracleError::BondBounds.into());
        }

        bond_mint = bond_currency.mint;
    }

    let request_index: u64;

    // Step 3: Get and increment next request index.
    {
        let mut oracle = OracleV1::from_account_info_mut(ctx.accounts.oracle)?;

        request_index = oracle.next_index;

        oracle.next_index = checked_add!(oracle.next_index, 1)?;
        oracle.save()?;
    }

    // Step 4: Check config.
    ConfigV1::from_account_info(ctx.accounts.config)?;

    // Step 5: Initialize request account.
    {
        let bump = pda::request::assert_pda(ctx.accounts.request.key, &request_index)?;
        let signer_seeds = pda::request::seeds_with_bump(&request_index, &bump);

        RequestV1::try_init(InitRequest {
            index: request_index,
            config: *ctx.accounts.config.key,
            creator: *ctx.accounts.creator.key,
            reward: args.reward,
            reward_mint: *ctx.accounts.reward_mint.key,
            bond: args.bond,
            bond_mint,
            timestamp: args.timestamp,
            arbitrator: args.arbitrator,
            data: args.data,
        })?
        .save(InitContext {
            account: ctx.accounts.request,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    // Step 6: Transfer reward to escrow.
    if args.reward > 0 {
        let mint_decimals = cpi::spl::mint_decimals(ctx.accounts.reward_mint)?;

        // Step 6.1: Initialize `reward_escrow` account.
        {
            let bump =
                pda::reward::assert_pda(ctx.accounts.reward_escrow.key, ctx.accounts.request.key)?;
            let signer_seeds = pda::reward::seeds_with_bump(ctx.accounts.request.key, &bump);

            cpi::spl::create_token_account(
                ctx.accounts.request.key,
                cpi::spl::CreateTokenAccount {
                    account: ctx.accounts.reward_escrow,
                    mint: ctx.accounts.reward_mint,
                    payer: ctx.accounts.payer,
                    token_program: ctx.accounts.token_program,
                    system_program: ctx.accounts.system_program,
                },
                &[&signer_seeds],
            )?;
        }

        // Step 6.2: Transfer reward from `reward_source` to `reward_escrow`.
        cpi::spl::transfer_checked(
            args.reward,
            mint_decimals,
            cpi::spl::TransferChecked {
                source: ctx.accounts.reward_source,
                destination: ctx.accounts.reward_escrow,
                mint: ctx.accounts.reward_mint,
                authority: ctx.accounts.creator,
                token_program: ctx.accounts.token_program,
            },
            &[],
        )?;
    }

    // TODO: Emit an event?

    Ok(())
}
