use borsh::BorshDeserialize;
use common::VariantName;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

mod claim_assertion_v1;
mod claim_dispute_v1;
mod claim_vote_v1;
mod close_voting_v1;
mod create_assertion_v1;
mod create_config_v1;
mod create_currency_v1;
mod create_oracle_v1;
mod create_request_v1;
mod create_stake_v1;
mod dispute_assertion_v1;
mod resolve_assertion_v1;
mod submit_vote_v1;
mod update_config_v1;
mod update_currency_v1;
mod update_oracle_v1;

pub(crate) use self::claim_assertion_v1::*;
pub(crate) use self::claim_dispute_v1::*;
pub(crate) use self::claim_vote_v1::*;
pub(crate) use self::close_voting_v1::*;
pub(crate) use self::create_assertion_v1::*;
pub(crate) use self::create_config_v1::*;
pub(crate) use self::create_currency_v1::*;
pub(crate) use self::create_oracle_v1::*;
pub(crate) use self::create_request_v1::*;
pub(crate) use self::create_stake_v1::*;
pub(crate) use self::dispute_assertion_v1::*;
pub(crate) use self::resolve_assertion_v1::*;
pub(crate) use self::submit_vote_v1::*;
pub(crate) use self::update_config_v1::*;
pub(crate) use self::update_currency_v1::*;
pub(crate) use self::update_oracle_v1::*;

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    use crate::instruction::OracleInstruction as I;

    let instruction = I::try_from_slice(instruction_data)?;

    log!("Instruction: {}", instruction.variant_name());

    match instruction {
        I::CreateOracleV1(args) => create_oracle_v1(program_id, accounts, args),
        I::UpdateOracleV1(args) => update_oracle_v1(program_id, accounts, args),
        I::CreateConfigV1(args) => create_config_v1(program_id, accounts, args),
        I::UpdateConfigV1(args) => update_config_v1(program_id, accounts, args),
        I::CreateCurrencyV1(args) => create_currency_v1(program_id, accounts, args),
        I::UpdateCurrencyV1(args) => update_currency_v1(program_id, accounts, args),
        I::CreateRequestV1(args) => create_request_v1(program_id, accounts, args),
        I::CreateAssertionV1(args) => create_assertion_v1(program_id, accounts, args),
        I::ResolveAssertionV1 => resolve_assertion_v1(program_id, accounts),
        I::DisputeAssertionV1 => dispute_assertion_v1(program_id, accounts),
        I::SubmitVoteV1(args) => submit_vote_v1(program_id, accounts, args),
        I::CloseVotingV1 => close_voting_v1(program_id, accounts),
        I::CreateStakeV1(args) => create_stake_v1(program_id, accounts, args),
        I::ClaimAssertionV1 => claim_assertion_v1(program_id, accounts),
        I::ClaimDisputeV1 => claim_dispute_v1(program_id, accounts),
        I::ClaimVoteV1 => claim_vote_v1(program_id, accounts),
    }
}
