use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::UpdateConfigV1Accounts;
use crate::state::{AccountSized, ConfigV1};
use crate::utils;

#[derive(Clone, BorshDeserialize)]
pub enum UpdateConfigV1Args {
    Authority {
        new_authority: Pubkey,
    },
    Config {
        new_bond_fee_bps: u16,

        new_dispute_window: u32,
        new_voting_window: u32,
        new_arbitration_window: u32,
    },
}

pub fn update_config_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: UpdateConfigV1Args,
) -> ProgramResult {
    let ctx = UpdateConfigV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.authority)?;

    // Step 1: Update config.
    {
        let mut config = ConfigV1::from_account_info_mut(ctx.accounts.config)?;

        // Guard config authority.
        config.assert_authority(ctx.accounts.authority.key)?;

        match args {
            UpdateConfigV1Args::Authority { new_authority } => {
                config.authority = new_authority;
            }
            UpdateConfigV1Args::Config {
                new_bond_fee_bps,
                new_dispute_window,
                new_voting_window,
                new_arbitration_window,
            } => {
                config.bond_fee_bps = new_bond_fee_bps;

                config.dispute_window = new_dispute_window;
                config.voting_window = new_voting_window;
                config.arbitration_window = new_arbitration_window;
            }
        }

        config.save()?;
    }

    Ok(())
}
