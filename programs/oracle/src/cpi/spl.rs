#![allow(dead_code)]

use std::cell::Ref;
use std::mem;
use std::ops::Deref;

use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{get_return_data, invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use spl_token_2022::extension::{AccountType, BaseState, BaseStateWithExtensions};
use spl_token_2022::state::Multisig;

use crate::utils;

pub type Mint<'a> = SplExtAccount<'a, spl_token_2022::state::Mint>;
pub type TokenAccount<'a> = SplExtAccount<'a, spl_token_2022::state::Account>;

const MINT_DECIMALS_OFFSET: usize = 44;

pub fn get_decimals(mint: &AccountInfo) -> Result<u8, ProgramError> {
    let data = mint.try_borrow_data()?;
    if data.len() < Mint::LEN {
        err!("Invalid mint account");
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(data[MINT_DECIMALS_OFFSET])
}

pub struct SplExtAccount<'a, S: BaseState> {
    base: S,
    tlv_data: Ref<'a, [u8]>,
}

impl<S: BaseState> Deref for SplExtAccount<'_, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<S: BaseState> BaseStateWithExtensions<S> for SplExtAccount<'_, S> {
    fn get_tlv_data(&self) -> &[u8] {
        &self.tlv_data
    }
}

const BASE_ACCOUNT_LENGTH: usize = spl_token_2022::state::Account::LEN;

trait Offsets: BaseState {
    const ACCOUNT_TYPE_REST_OFFSET: usize = BASE_ACCOUNT_LENGTH - Self::LEN;
    const TLV_START_REST_OFFSET: usize =
        Self::ACCOUNT_TYPE_REST_OFFSET + mem::size_of::<AccountType>();

    const TLV_START_OFFSET: usize = Self::LEN + Self::TLV_START_REST_OFFSET;
}

impl<S: BaseState> Offsets for S {}

impl<'a, S: BaseState> SplExtAccount<'a, S> {
    pub const LEN: usize = <S as Pack>::LEN;

    pub fn from_account_info<'b: 'a>(info: &'a AccountInfo<'b>) -> Result<Self, ProgramError> {
        let data_ref: Ref<'a, &mut [u8]> = info.try_borrow_data()?;
        let data: &[u8] = *data_ref;

        // Multisig doesn't support extensions.
        if data.len() == Multisig::LEN || data.len() < S::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        // Invariant: data.len() >= S::LEN

        let (base_data, rest) = data.split_at(S::LEN);
        // Invariant: base_data.len() = S::LEN
        //
        //            data.len() = base_data.len() + rest.len()
        //                       = S::LEN + rest.len()

        let base = S::unpack(base_data)?;

        if rest.is_empty() {
            return Ok(Self { base, tlv_data: Ref::map(data_ref, |_| &[]) });
        }

        if rest.len() <= S::TLV_START_REST_OFFSET {
            return Err(ProgramError::InvalidAccountData);
        }
        // Invariant: rest.len() > S::TLV_START_REST_OFFSET

        // Don't bother to check if padding is zeroed in release mode,
        // since it will be checked by the SPL token program.
        if cfg!(debug_assertions) && S::ACCOUNT_TYPE_REST_OFFSET > 0 {
            let padding = &rest[..S::ACCOUNT_TYPE_REST_OFFSET];
            if !padding.iter().all(|&v| v == 0) {
                return Err(ProgramError::InvalidAccountData);
            }
        }

        let tlv_data = Ref::map(data_ref, |data| {
            // SAFETY:
            //
            // We use unsafe here since the compiler can't infer that `data` here
            // refers to the same slice as `data` outside of the `Ref::map`. This
            // causes the compiler to fail to optimize the panic branch out for
            // the expression:
            //
            //     data[S::TLV_START_OFFSET..]
            //
            // We can guarantee that this index is valid by checking that it is
            // less than the length of `data`:
            //
            //     S::TLV_START_OFFSET = S::LEN + S::TLV_START_REST_OFFSET
            //
            //     base_data.len() = S::LEN
            //     rest.len() > S::TLV_START_REST_OFFSET
            //
            //     data.len() = base_data.len() + rest.len()
            //                = S::LEN + rest.len()
            //
            //     data.len() > S::LEN + S::TLV_START_REST_OFFSET
            //
            //     data.len() > S::TLV_START_OFFSET
            //
            // Therefore the use of `get_unchecked` is safe.
            unsafe { data.get_unchecked(S::TLV_START_OFFSET..) }
        });

        Ok(Self { base, tlv_data })
    }
}

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

/// Creates a new token account.
pub fn create_token_account(
    owner: &Pubkey,
    accounts: CreateTokenAccount,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let CreateTokenAccount { account, mint, payer, token_program, system_program } = accounts;

    let account_len = get_account_len(mint, token_program)?;

    utils::create_or_allocate_account(
        account,
        payer,
        system_program,
        account_len,
        token_program.key,
        signer_seeds,
    )?;

    invoke_signed(
        &spl_token_2022::instruction::initialize_account3(
            token_program.key,
            account.key,
            mint.key,
            owner,
        )?,
        &[account.clone(), mint.clone()],
        signer_seeds,
    )?;

    Ok(())
}

/// Transfers tokens from the source account to the destination account.
pub fn transfer_checked(
    amount: u64,
    decimals: u8,
    accounts: TransferChecked,
    signer_seeds: &[&[&[u8]]],
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
        signer_seeds,
    )?;

    Ok(())
}

/// Transfers tokens from the source account to the destination account.
pub fn close_account(accounts: CloseAccount, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
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
        signer_seeds,
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
        &[mint.clone(), token_program.clone()],
    )?;
    get_return_data().ok_or(ProgramError::InvalidInstructionData).and_then(|(key, data)| {
        if key != *token_program.key {
            return Err(ProgramError::IncorrectProgramId);
        }
        data.try_into().map(usize::from_le_bytes).map_err(|_| ProgramError::InvalidInstructionData)
    })
}
