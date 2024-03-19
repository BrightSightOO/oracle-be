use borsh::{BorshDeserialize, BorshSerialize};
use common::{BuildInstruction, VariantName};
use shank::{ShankContext, ShankInstruction};
use solana_program::pubkey::Pubkey;

use crate::state::RequestData;

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

    /// Creates a new [`Request`].
    ///
    /// [`Request`]: crate::state::Request
    #[account(0, writable, name = "oracle", desc = "Program oracle account")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, name = "reward_mint", desc = "Reward mint")]
    #[account(3, writable, name = "reward_source", desc = "Reward source token account")]
    #[account(4, writable, name = "reward_escrow", desc = "Reward escrow token account")]
    #[account(5, signer, name = "creator", desc = "Creator")]
    #[account(6, signer, writable, name = "payer", desc = "Payer")]
    #[account(7, name = "token_program", desc = "SPL token program")]
    #[account(8, name = "system_program", desc = "System program")]
    CreateRequest(CreateRequestArgs),
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum CreateOracleArgs {
    V1 {
        /// Authority.
        authority: Pubkey,
    },
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum CreateRequestArgs {
    V1 {
        /// Amount rewarded to the asserter/disputer on resolution.
        reward: u64,
        /// Unix timestamp after which a value can be asserted.
        timestamp: i64,
        /// Request data.
        data: RequestData,
    },
}
