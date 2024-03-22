use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program::invoke_signed;
use solana_program::pubkey::Pubkey;

pub const ID: Pubkey = solana_program::pubkey!("Cf9JrByfmw6CYSry39pfg2BSGHRgde2Cp5y6yZ3a2Yeo");

#[derive(Clone, Copy, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
#[repr(u8)]
pub enum Outcome {
    Open,
    Yes,
    No,
    Invalid,
}

pub struct UpdateState<'a, 'info> {
    pub market: &'a AccountInfo<'info>,
    pub resolver: &'a AccountInfo<'info>,
    pub parimutuel_program: &'a AccountInfo<'info>,
}

pub fn update_state(
    outcome: Outcome,
    accounts: UpdateState,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    // name = "update_state"
    // preimage = "global:{name}"
    // hash = sha256(preimage)
    // discriminator = hash[0..8]
    const UPDATE_STATE_DISCRIMINATOR: [u8; 8] = [0x87, 0x70, 0xd7, 0x4b, 0xf7, 0xb9, 0x35, 0xb0];

    let UpdateState { market, resolver, parimutuel_program } = accounts;

    let mut data = Vec::with_capacity(UPDATE_STATE_DISCRIMINATOR.len() + 1);

    data.extend_from_slice(&UPDATE_STATE_DISCRIMINATOR);
    data.push(outcome as u8);

    let instruction = Instruction {
        program_id: *parimutuel_program.key,
        accounts: vec![
            AccountMeta::new(*market.key, false),
            AccountMeta::new_readonly(*resolver.key, true),
        ],
        data,
    };

    invoke_signed(&instruction, &[market.clone(), resolver.clone()], signer_seeds)
}
