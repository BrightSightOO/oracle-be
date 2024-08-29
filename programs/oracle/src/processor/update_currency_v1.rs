use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::UpdateCurrencyV1Accounts;
use crate::state::{Account, AccountSized, ConfigV1, CurrencyV1};
use crate::utils;
use crate::utils::Bounds;

#[derive(Clone, BorshDeserialize)]
pub struct UpdateCurrencyV1Args {
    pub new_reward_range: Bounds,
    pub new_bond_range: Bounds,
}

pub fn update_currency_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: UpdateCurrencyV1Args,
) -> ProgramResult {
    let ctx = UpdateCurrencyV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.authority)?;

    // Step 1: Check config authority.
    ConfigV1::from_account_info(ctx.accounts.config)?
        .assert_authority(ctx.accounts.authority.key)?;

    // Step 1: Update currency.
    {
        let mut currency = CurrencyV1::from_account_info_mut(ctx.accounts.currency)?;

        // Guard currency.
        currency.assert_pda(ctx.accounts.currency.key)?;
        currency.assert_config(ctx.accounts.config.key)?;

        currency.reward_range = args.new_reward_range;
        currency.bond_range = args.new_bond_range;

        currency.save()?;
    }

    Ok(())
}
