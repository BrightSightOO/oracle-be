#![allow(dead_code)]

use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;

pub struct Transfer<'a, 'info> {
    pub source: &'a AccountInfo<'info>,
    pub destination: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

/// Transfers lamports from the source account to the destination account.
pub fn transfer(lamports: u64, accounts: Transfer, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
    let Transfer { source, destination, system_program } = accounts;

    invoke_signed(
        &system_instruction::transfer(source.key, destination.key, lamports),
        &[source.clone(), destination.clone(), system_program.clone()],
        signer_seeds,
    )?;

    Ok(())
}

pub struct Assign<'a, 'info> {
    pub account: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

/// Assign ownership of an account.
pub fn assign(owner: &Pubkey, accounts: Assign, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
    let Assign { account, system_program } = accounts;

    invoke_signed(
        &system_instruction::assign(account.key, owner),
        &[account.clone(), system_program.clone()],
        signer_seeds,
    )?;

    Ok(())
}
