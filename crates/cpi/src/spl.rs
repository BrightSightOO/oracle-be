#![allow(dead_code)]

use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::get_return_data;
use solana_program::program_error::ProgramError;
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;
use solana_utils::invoke::{invoke, invoke_signed};
use spl_token_2022::extension::PodStateWithExtensions;
use spl_token_2022::instruction::AuthorityType;
use spl_token_2022::pod::{PodAccount, PodMint};

pub const TOKEN_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const TOKEN_2022_ID: Pubkey = spl_token_2022::ID;

pub const ASSOCIATED_TOKEN_ID: Pubkey = pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

pub struct CreateTokenAccount<'a, 'info> {
    pub account: &'a AccountInfo<'info>,
    pub mint: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
}

pub struct TransferChecked<'a, 'info> {
    pub source: &'a AccountInfo<'info>,
    pub destination: &'a AccountInfo<'info>,
    pub mint: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
}

pub struct CloseAccount<'a, 'info> {
    pub account: &'a AccountInfo<'info>,
    pub destination: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
}

pub struct SetAuthority<'a, 'info> {
    pub owned: &'a AccountInfo<'info>,
    pub owner: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
}

pub struct ApproveDelegate<'a, 'info> {
    pub account: &'a AccountInfo<'info>,
    pub delegate: &'a AccountInfo<'info>,
    pub owner: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
}

/// Creates a new token account.
pub fn create_token_account(
    owner: &Pubkey,
    accounts: CreateTokenAccount,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let CreateTokenAccount { account, mint, payer, token_program, system_program } = accounts;

    let account_len = get_account_len(mint, token_program)?;

    solana_utils::create_or_allocate_account(
        account,
        payer,
        system_program,
        account_len,
        token_program.key,
        signers_seeds,
    )?;

    invoke_signed(
        &spl_token_2022::instruction::initialize_account3(
            token_program.key,
            account.key,
            mint.key,
            owner,
        )?,
        &[account.clone(), mint.clone()],
        signers_seeds,
    )?;

    Ok(())
}

/// Transfers tokens from the source account to the destination account.
pub fn transfer_checked(
    amount: u64,
    decimals: u8,
    accounts: TransferChecked,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let TransferChecked { source, destination, mint, authority, token_program } = accounts;

    invoke_signed(
        &spl_token_2022::instruction::transfer_checked(
            token_program.key,
            source.key,
            mint.key,
            destination.key,
            authority.key,
            &[],
            amount,
            decimals,
        )?,
        &[source.clone(), mint.clone(), destination.clone(), authority.clone()],
        signers_seeds,
    )?;

    Ok(())
}

/// Closes token account and transfers rent to the destination account.
pub fn close_account(accounts: CloseAccount, signers_seeds: &[&[&[u8]]]) -> ProgramResult {
    let CloseAccount { account, destination, authority, token_program } = accounts;

    invoke_signed(
        &spl_token_2022::instruction::close_account(
            token_program.key,
            account.key,
            destination.key,
            authority.key,
            &[],
        )?,
        &[account.clone(), destination.clone(), authority.clone()],
        signers_seeds,
    )?;

    Ok(())
}

/// Sets a new authority for an account or mint.
pub fn set_authority(
    new_authority: Option<&Pubkey>,
    authority_type: AuthorityType,
    accounts: SetAuthority,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let SetAuthority { owned, owner, token_program } = accounts;

    invoke_signed(
        &spl_token_2022::instruction::set_authority(
            token_program.key,
            owned.key,
            new_authority,
            authority_type,
            owner.key,
            &[],
        )?,
        &[owned.clone(), owner.clone()],
        signers_seeds,
    )?;

    Ok(())
}

/// Approves a delegate.
pub fn approve_delegate(
    amount: u64,
    accounts: ApproveDelegate,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ApproveDelegate { account, delegate, owner, token_program } = accounts;

    invoke_signed(
        &spl_token_2022::instruction::approve(
            token_program.key,
            account.key,
            delegate.key,
            owner.key,
            &[],
            amount,
        )?,
        &[account.clone(), delegate.clone(), owner.clone()],
        signers_seeds,
    )?;

    Ok(())
}

/// Determines the required initial data length for a new token account.
fn get_account_len<'a>(
    mint: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
) -> Result<usize, ProgramError> {
    invoke(
        &spl_token_2022::instruction::get_account_data_size(token_program.key, mint.key, &[])?,
        &[mint.clone()],
    )?;
    get_return_data().ok_or(ProgramError::InvalidInstructionData).and_then(|(key, data)| {
        if !solana_utils::pubkeys_eq(&key, token_program.key) {
            return Err(ProgramError::IncorrectProgramId);
        }
        data.try_into().map(usize::from_le_bytes).map_err(|_| ProgramError::InvalidInstructionData)
    })
}

/// Gets the number of decimals in the mint.
pub fn mint_decimals(mint: &AccountInfo<'_>) -> Result<u8, ProgramError> {
    let data = mint.data.borrow();
    let mint = PodStateWithExtensions::<PodMint>::unpack(*data)?;

    Ok(mint.base.decimals)
}

/// Gets the amount of tokens in the account.
pub fn account_amount(account: &AccountInfo<'_>) -> Result<u64, ProgramError> {
    let data = account.data.borrow();
    let account = PodStateWithExtensions::<PodAccount>::unpack(*data)?;

    let amount = account.base.amount.0;
    let amount = u64::from_le_bytes(amount);

    Ok(amount)
}
