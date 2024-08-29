use borsh::BorshDeserialize;
use shank::{ShankContext, ShankInstruction};
use solana_utils::VariantName;

use crate::processor::*;

#[rustfmt::skip::attributes(account)]
#[derive(Clone, VariantName, ShankContext, ShankInstruction, BorshDeserialize)]
pub enum OracleInstruction {
    /// Creates program oracle.
    #[account(0, writable, name = "oracle", desc = "Oracle")]
    #[account(1, name = "governance_mint", desc = "Governance token mint")]
    #[account(2, signer, writable, name = "payer", desc = "Payer")]
    #[account(3, name = "system_program", desc = "System program")]
    CreateOracleV1(CreateOracleV1Args),

    /// Updates program oracle.
    #[account(0, writable, name = "oracle", desc = "Oracle")]
    #[account(1, signer, name = "authority", desc = "Oracle authority")]
    UpdateOracleV1(UpdateOracleV1Args),

    /// Creates config.
    #[account(0, signer, writable, name = "config", desc = "Config")]
    #[account(1, signer, writable, name = "payer", desc = "Payer")]
    #[account(2, name = "system_program", desc = "System program")]
    CreateConfigV1(CreateConfigV1Args),

    /// Updates config.
    #[account(0, writable, name = "config", desc = "Config")]
    #[account(1, signer, name = "authority", desc = "Config authority")]
    UpdateConfigV1(UpdateConfigV1Args),

    /// Creates a currency.
    #[account(0, name = "config", desc = "Config")]
    #[account(1, writable, name = "currency", desc = "Currency")]
    #[account(2, name = "mint", desc = "Mint")]
    #[account(3, name = "authority", desc = "Oracle authority")]
    #[account(4, name = "payer", desc = "Payer")]
    #[account(5, name = "token_program", desc = "SPL token program")]
    #[account(6, name = "system_program", desc = "System program")]
    CreateCurrencyV1(CreateCurrencyV1Args),

    /// Updates a currency.
    #[account(0, name = "config", desc = "Config")]
    #[account(1, writable, name = "currency", desc = "Currency")]
    #[account(2, name = "authority", desc = "Oracle authority")]
    UpdateCurrencyV1(UpdateCurrencyV1Args),

    /// Creates a new request.
    #[account(0, writable, name = "oracle", desc = "Oracle")]
    #[account(1, name = "config", desc = "Config")]
    #[account(2, writable, name = "request", desc = "Request")]
    #[account(3, name = "reward_currency", desc = "Reward currency")]
    #[account(4, name = "bond_currency", desc = "Bond currency")]
    #[account(5, name = "reward_mint", desc = "Reward mint")]
    #[account(6, writable, name = "reward_source", desc = "Reward source token account")]
    #[account(7, writable, name = "reward_escrow", desc = "Reward escrow token account")]
    #[account(8, signer, name = "creator", desc = "Creator")]
    #[account(9, signer, writable, name = "payer", desc = "Payer")]
    #[account(10, name = "token_program", desc = "SPL token program")]
    #[account(11, name = "system_program", desc = "System program")]
    CreateRequestV1(CreateRequestV1Args),

    /// Creates an assertion for a request.
    #[account(0, name = "config", desc = "Config")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, writable, name = "assertion", desc = "Assertion")]
    #[account(3, name = "bond_mint", desc = "Bond mint")]
    #[account(4, writable, name = "bond_source", desc = "Bond source token account")]
    #[account(5, writable, name = "bond_escrow", desc = "Bond escrow token account")]
    #[account(6, signer, name = "asserter", desc = "Asserter")]
    #[account(7, signer, writable, name = "payer", desc = "Payer")]
    #[account(8, name = "token_program", desc = "SPL token program")]
    #[account(9, name = "system_program", desc = "System program")]
    CreateAssertionV1(CreateAssertionV1Args),

    /// Resolves an undisputed assertion after the expiration timestamp.
    #[account(0, name = "config", desc = "Config")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, name = "assertion", desc = "Assertion")]
    ResolveAssertionV1,

    /// Disputes the assertion for a request.
    #[account(0, name = "oracle", desc = "Oracle")]
    #[account(1, name = "config", desc = "Config")]
    #[account(2, writable, name = "request", desc = "Request")]
    #[account(3, writable, name = "assertion", desc = "Assertion")]
    #[account(4, writable, name = "voting", desc = "Voting")]
    #[account(5, name = "bond_mint", desc = "Bond mint")]
    #[account(6, writable, name = "bond_source", desc = "Bond source token account")]
    #[account(7, writable, name = "bond_escrow", desc = "Bond escrow token account")]
    #[account(8, signer, name = "disputer", desc = "Disputer")]
    #[account(9, signer, writable, name = "payer", desc = "Payer")]
    #[account(10, name = "token_program", desc = "SPL token program")]
    #[account(11, name = "system_program", desc = "System program")]
    DisputeAssertionV1,

    /// Submits a vote for resolving a disputed assertion.
    #[account(0, name = "config", desc = "Config")]
    #[account(1, name = "request", desc = "Request")]
    #[account(2, writable, name = "voting", desc = "Voting")]
    #[account(3, writable, name = "vote", desc = "Vote")]
    #[account(4, name = "stake", desc = "Stake")]
    #[account(5, signer, name = "voter", desc = "Voter")]
    #[account(6, signer, writable, name = "payer", desc = "Payer")]
    #[account(7, name = "system_program", desc = "System program")]
    SubmitVoteV1(SubmitVoteV1Args),

    /// Closes voting and resolves the request.
    #[account(0, name = "config", desc = "Config")]
    #[account(1, writable, name = "request", desc = "Request")]
    #[account(2, writable, name = "voting", desc = "Voting")]
    CloseVotingV1,

    /// Creates a stake account.
    #[account(0, name = "oracle", desc = "Oracle")]
    #[account(1, signer, writable, name = "stake", desc = "Stake")]
    #[account(2, writable, name = "mint", desc = "Stake")]
    #[account(3, writable, name = "stake_source", desc = "Stake source token account")]
    #[account(4, writable, name = "stake_pool", desc = "Stake pool token account")]
    #[account(5, signer, writable, name = "wallet", desc = "Stake owner")]
    #[account(6, signer, writable, name = "payer", desc = "Payer")]
    #[account(7, name = "token_program", desc = "SPL token program")]
    #[account(8, name = "system_program", desc = "System program")]
    CreateStakeV1(CreateStakeV1Args),

    #[account(0, name = "request", desc = "Request")]
    #[account(1, writable, name = "assertion", desc = "Assertion")]
    #[account(2, name = "bond_mint", desc = "Bond mint")]
    #[account(3, writable, name = "bond_destination", desc = "Reclaimed bond destination token account")]
    #[account(4, writable, name = "bond_escrow", desc = "Asserter bond escrow token account")]
    #[account(5, name = "reward_mint", desc = "Reward mint")]
    #[account(6, writable, name = "reward_destination", desc = "Reward destination token account")]
    #[account(7, writable, name = "reward_escrow", desc = "Reward escrow token account")]
    #[account(8, signer, writable, name = "asserter", desc = "Asserter")]
    #[account(9, name = "token_program", desc = "SPL token program")]
    #[account(10, name = "system_program", desc = "System program")]
    ClaimAssertionV1,

    #[account(0, name = "request", desc = "Request")]
    #[account(1, writable, name = "assertion", desc = "Assertion")]
    #[account(2, name = "bond_mint", desc = "Bond mint")]
    #[account(3, writable, name = "bond_destination", desc = "Reclaimed bond destination token account")]
    #[account(4, writable, name = "bond_escrow", desc = "Disputer bond escrow token account")]
    #[account(5, name = "reward_mint", desc = "Reward mint")]
    #[account(6, writable, name = "reward_destination", desc = "Reward destination token account")]
    #[account(7, writable, name = "reward_escrow", desc = "Reward escrow token account")]
    #[account(8, signer, writable, name = "disputer", desc = "Disputer")]
    #[account(9, name = "token_program", desc = "SPL token program")]
    #[account(10, name = "system_program", desc = "System program")]
    ClaimDisputeV1,

    #[account(0, name = "request", desc = "Request")]
    #[account(1, name = "assertion", desc = "Assertion")]
    #[account(2, name = "voting", desc = "Voting")]
    #[account(3, writable, name = "vote", desc = "Vote")]
    #[account(4, name = "stake", desc = "Stake")]
    #[account(5, name = "bond_mint", desc = "Bond mint")]
    #[account(6, writable, name = "bond_destination", desc = "Bond destination token account")]
    #[account(7, writable, name = "bond_escrow", desc = "Bond escrow token account of incorrect asserter/disputer")]
    #[account(8, signer, writable, name = "voter", desc = "Voter")]
    #[account(9, name = "token_program", desc = "SPL token program")]
    #[account(10, name = "system_program", desc = "System program")]
    ClaimVoteV1,
}
