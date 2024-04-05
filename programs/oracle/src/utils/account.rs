#![allow(dead_code)]

use std::cell::RefMut;

use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::{ProgramResult, MAX_PERMITTED_DATA_INCREASE};
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{system_instruction, system_program};

use crate::error::OracleError;

/// Create an account.
pub fn create_or_allocate_account<'a>(
    account: &AccountInfo<'a>,
    payer: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    space: usize,
    owner: &Pubkey,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    if !common::cmp_pubkeys(account.owner, &system_program::ID) {
        err!("Account {} is already initialized", account.key);
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(space).max(1);

    let lamports = account.try_lamports()?;
    if lamports > 0 {
        let required_lamports = required_lamports.saturating_sub(lamports);

        if required_lamports > 0 {
            invoke(
                &system_instruction::transfer(payer.key, account.key, required_lamports),
                &[payer.clone(), account.clone(), system_program.clone()],
            )?;
        }

        let accounts = &[account.clone(), system_program.clone()];

        invoke_signed(
            &system_instruction::allocate(account.key, space as u64),
            accounts,
            signer_seeds,
        )?;

        invoke_signed(&system_instruction::assign(account.key, owner), accounts, signer_seeds)?;
    } else {
        invoke_signed(
            &system_instruction::create_account(
                payer.key,
                account.key,
                required_lamports,
                space as u64,
                owner,
            ),
            &[payer.clone(), account.clone(), system_program.clone()],
            signer_seeds,
        )?;
    }

    Ok(())
}

/// Close `account` and transfer lamports to `sol_dst`.
pub fn close_account<'a>(account: &AccountInfo<'a>, sol_dst: &AccountInfo<'a>) -> ProgramResult {
    let mut src_lamports = account.try_borrow_mut_lamports()?;
    let mut dst_lamports = sol_dst.try_borrow_mut_lamports()?;

    let src_lamports = &mut **src_lamports;
    let dst_lamports = &mut **dst_lamports;

    let lamports =
        dst_lamports.checked_add(*src_lamports).ok_or(OracleError::ArithmeticOverflow)?;

    *dst_lamports = lamports;
    *src_lamports = 0;

    account.assign(&system_program::ID);
    account.realloc(0, false)?;

    Ok(())
}

/// Resize an account.
pub fn realloc_account_mut<'a>(
    info: &'a AccountInfo,
    new_len: usize,
) -> Result<RefMut<'a, [u8]>, ProgramError> {
    let mut data = info.try_borrow_mut_data()?;
    let old_len = data.len();

    // Return early if length hasn't changed.
    if new_len == old_len {
        return Ok(RefMut::map(data, |data| &mut data[..new_len]));
    }

    // Return early if the length increase from the original serialized data
    // length is too large and would result in an out of bounds allocation.
    let original_data_len = unsafe { info.original_data_len() };
    if new_len.saturating_sub(original_data_len) > MAX_PERMITTED_DATA_INCREASE {
        return Err(ProgramError::InvalidRealloc);
    }

    // Reallocate.
    unsafe {
        let data_ptr = data.as_mut_ptr();

        // First set new length in the serialized data.
        *(data_ptr.offset(-8) as *mut u64) = new_len as u64;

        // Then recreate the local slice with the new length.
        *data = std::slice::from_raw_parts_mut(data_ptr, new_len)
    }

    Ok(RefMut::map(data, |data| &mut data[..new_len]))
}

/// Transfer lamports from `src` to `dst`, where `src` is owned by the executing program.
pub fn transfer_lamports(src: &AccountInfo, dst: &AccountInfo, amount: u64) -> ProgramResult {
    let mut src_lamports = src.try_borrow_mut_lamports()?;
    let mut dst_lamports = dst.try_borrow_mut_lamports()?;

    let src_lamports = &mut **src_lamports;
    let dst_lamports = &mut **dst_lamports;

    let final_src_lamports =
        src_lamports.checked_sub(amount).ok_or(ProgramError::InsufficientFunds)?;
    let final_dst_lamports =
        dst_lamports.checked_add(amount).ok_or(OracleError::ArithmeticOverflow)?;

    *src_lamports = final_src_lamports;
    *dst_lamports = final_dst_lamports;

    Ok(())
}
