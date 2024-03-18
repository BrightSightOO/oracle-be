use borsh::{BorshDeserialize, BorshSerialize};
use common::{BuildInstruction, VariantName};
use shank::{ShankContext, ShankInstruction};
use solana_program::pubkey::Pubkey;

#[rustfmt::skip::attributes(account)]
#[derive(
    Clone,
    VariantName,
    BuildInstruction,
    ShankContext,
    ShankInstruction,
    BorshDeserialize,
    BorshSerialize,
)]
pub enum OracleInstruction {
    /// Creates program [`Oracle`].
    ///
    /// [`Oracle`]: crate::state::Oracle
    #[account(0, writable, name = "oracle", desc = "Program oracle account")]
    #[account(1, signer, writable, name = "payer", desc = "Payer")]
    #[account(2, name = "system_program", desc = "System program")]
    CreateOracle(CreateOracleArgs),
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum CreateOracleArgs {
    V1 {
        /// Authority.
        authority: Pubkey,
    },
}
