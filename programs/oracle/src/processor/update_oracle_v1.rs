use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::UpdateOracleV1Accounts;
use crate::state::{AccountSized, OracleV1};
use crate::{pda, utils};

#[derive(Clone, BorshDeserialize)]
pub enum UpdateOracleV1Args {
    Authority { new_authority: Pubkey },
}

pub fn update_oracle_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: UpdateOracleV1Args,
) -> ProgramResult {
    let ctx = UpdateOracleV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.authority)?;

    // Guard PDAs.
    pda::oracle::assert_pda(ctx.accounts.oracle.key)?;

    // Step 1: Update oracle.
    {
        let mut oracle = OracleV1::from_account_info_mut(ctx.accounts.oracle)?;

        // Guard oracle authority.
        oracle.assert_authority(ctx.accounts.authority.key)?;

        match args {
            UpdateOracleV1Args::Authority { new_authority } => {
                oracle.authority = new_authority;
            }
        }

        oracle.save()?;
    }

    Ok(())
}
