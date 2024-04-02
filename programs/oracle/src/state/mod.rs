use std::cell::RefMut;
use std::convert::Infallible;
use std::ops::{Deref, DerefMut};

use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;
use crate::utils;

mod assertion;
mod currency;
mod oracle;
mod request;
mod stake;

pub use self::assertion::*;
pub use self::oracle::*;
pub use self::request::*;
pub use self::stake::*;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Debug,
    Default,
    FromPrimitive,
    BorshDeserialize,
    BorshSerialize,
    BorshSize,
)]
#[repr(u8)]
pub enum AccountType {
    /// Uninitialized account, which has all bytes set to zero by default.
    #[default]
    Uninitialized,
    /// Account containing [`Oracle`] state.
    Oracle,
    /// Account containing [`Stake`] state.
    Stake,
    /// Account containing [`Request`] state.
    Request,
    /// Account containing [`Assertion`] state.
    Assertion,
    /// Account containing [`Currency`] state.
    Currency,
}

pub(crate) trait Account: BorshDeserialize + BorshSerialize {
    const TYPE: AccountType;

    fn check_account_owner(owner: &Pubkey) -> Result<(), ProgramError> {
        if common::cmp_pubkeys(owner, &crate::ID) {
            Ok(())
        } else {
            err!("{:?} account is owned by the wrong program", Self::TYPE);
            Err(ProgramError::IncorrectProgramId)
        }
    }

    fn from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        let account_type = Self::TYPE;

        Self::deserialize(&mut &data[..]).map_err(|err| {
            err!("Failed to deserialize {account_type:?} account: {err}");
            ProgramError::InvalidAccountData
        })
    }

    fn safe_deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        let expected_type = Self::TYPE;

        let account_type = data.first().ok_or_else(|| {
            err!("{expected_type:?} account is uninitialized");
            ProgramError::UninitializedAccount
        })?;

        let account_type = AccountType::from_u8(*account_type).ok_or_else(|| {
            err!("Unknown account type: {account_type:#x}, expected {expected_type:?}");
            ProgramError::InvalidAccountData
        })?;

        if account_type != expected_type {
            err!("Incorrect account type: expected {expected_type:?}, found {account_type:?}");
            return match account_type {
                AccountType::Uninitialized => Err(ProgramError::UninitializedAccount),
                _ => Err(ProgramError::InvalidAccountData),
            };
        }

        Self::from_bytes(data)
    }

    fn from_account_info(info: &AccountInfo) -> Result<Self, ProgramError> {
        let data = info.try_borrow_data()?;
        let account = Self::safe_deserialize(*data)?;

        Self::check_account_owner(info.owner)?;

        Ok(account)
    }
}

pub(crate) trait AccountSized: Account {
    const IS_FIXED_SIZE: bool;

    fn serialized_size(&self) -> Option<usize>;

    fn from_account_info_mut<'a>(
        info: &'a AccountInfo<'a>,
    ) -> Result<AccountSizedMut<'a, Self>, ProgramError> {
        let data = info.try_borrow_mut_data()?;
        let account = Self::safe_deserialize(*data)?;

        Self::check_account_owner(info.owner)?;

        Ok(AccountSizedMut { data, account })
    }
}

impl<T: Account + BorshSize> AccountSized for T {
    const IS_FIXED_SIZE: bool = true;

    fn serialized_size(&self) -> Option<usize> {
        Some(<T as BorshSize>::SIZE)
    }
}

#[must_use = "Must call `.save()` to save account"]
pub(crate) struct AccountSizedMut<'a, T> {
    data: RefMut<'a, &'a mut [u8]>,
    account: T,
}

impl<'a, T: AccountSized> AccountSizedMut<'a, T> {
    pub fn save(mut self) -> ProgramResult {
        if !T::IS_FIXED_SIZE
            && self.serialized_size().ok_or(OracleError::ArithmeticOverflow)? > self.data.len()
        {
            err!("Account cannot be saved as it overflows allocation");
            return Err(ProgramError::InvalidAccountData);
        }
        BorshSerialize::serialize(&self.account, &mut *self.data)?;
        Ok(())
    }
}

impl<T> Deref for AccountSizedMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.account
    }
}

impl<T> DerefMut for AccountSizedMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.account
    }
}

pub(crate) trait InitAccount<Params, Error>: Account
where
    Params: TryInto<(Self, usize), Error = Error>,
{
    /// Returns the account and the space required for initialization.
    fn try_init(params: Params) -> Result<AccountInitializer<Self>, Error> {
        let (account, space) = params.try_into()?;
        Ok(AccountInitializer { account, space })
    }

    fn init(params: Params) -> AccountInitializer<Self>
    where
        Error: Into<Infallible>,
    {
        match Self::try_init(params).map_err(|err| err.into()) {
            Ok(account_init) => account_init,
            Err(err) => match err {},
        }
    }
}

impl<T, Params, Error> InitAccount<Params, Error> for T
where
    T: Account,
    Params: TryInto<(Self, usize), Error = Error>,
{
}

#[must_use = "Must call `.save()` to initialize account"]
pub(crate) struct AccountInitializer<T> {
    account: T,
    space: usize,
}

impl<T: Account> AccountInitializer<T> {
    pub fn save(self, context: InitContext) -> Result<T, ProgramError> {
        let InitContext { account: account_info, payer, system_program, program_id, signer_seeds } =
            context;

        utils::create_or_allocate_account(
            account_info,
            payer,
            system_program,
            self.space,
            program_id,
            signer_seeds,
        )?;

        let mut data = account_info.try_borrow_mut_data()?;

        self.account.serialize(&mut *data)?;

        Ok(self.account)
    }
}

impl<T> Deref for AccountInitializer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.account
    }
}

impl<T> DerefMut for AccountInitializer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.account
    }
}

pub(crate) struct InitContext<'a, 'b, 'c, 'info> {
    pub account: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
    pub program_id: &'a Pubkey,
    pub signer_seeds: &'a [&'b [&'c [u8]]],
}
