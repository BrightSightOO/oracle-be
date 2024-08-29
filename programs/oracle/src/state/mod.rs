use std::cell::RefMut;
use std::convert::Infallible;
use std::io;
use std::ops::{Deref, DerefMut};

use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::{BorshSize, BorshSizeProperties};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_utils::{log, VariantName};

mod assertion;
mod config;
mod currency;
mod oracle;
mod request;
mod stake;
mod vote;
mod voting;

use crate::error::OracleError;

pub use self::assertion::*;
pub use self::config::*;
pub use self::currency::*;
pub use self::oracle::*;
pub use self::request::*;
pub use self::stake::*;
pub use self::vote::*;
pub use self::voting::*;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    FromPrimitive,
    BorshDeserialize,
    BorshSerialize,
    BorshSize,
    VariantName,
)]
#[repr(u8)]
pub enum AccountType {
    /// Uninitialized account, which has all bytes set to zero by default.
    #[default]
    Uninitialized,
    /// Account containing [`OracleV1`] state.
    OracleV1,
    /// Account containing [`ConfigV1`] state.
    ConfigV1,
    /// Account containing [`StakeV1`] state.
    StakeV1,
    /// Account containing [`RequestV1`] state.
    RequestV1,
    /// Account containing [`AssertionV1`] state.
    AssertionV1,
    /// Account containing [`CurrencyV1`] state.
    CurrencyV1,
    /// Account containing [`VotingV1`] state.
    VotingV1,
    /// Account containing [`VoteV1`] state.
    VoteV1,
}

pub(crate) trait Account: BorshDeserialize + BorshSerialize {
    const TYPE: AccountType;

    #[inline]
    fn name() -> &'static str {
        Self::TYPE.variant_name()
    }

    fn check_account_owner(owner: &Pubkey) -> Result<(), ProgramError> {
        if !solana_utils::pubkeys_eq(owner, &crate::ID) {
            log!("Error: {} account is owned by the wrong program", Self::name());
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }

    fn safe_deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        let key = match data {
            [] | [0, ..] => {
                log!("Error: {} account is uninitialized", Self::name());
                return Err(OracleError::DeserializationError.into());
            }
            &[key, ..] => key,
        };

        let Some(account_type) = AccountType::from_u8(key) else {
            log!("Error: Unknown account type: {key:#x}, expected {}", Self::name());
            return Err(OracleError::DeserializationError.into());
        };

        if account_type != Self::TYPE {
            log!(
                "Error: Incorrect account type: {}, expected {}",
                account_type.variant_name(),
                Self::name(),
            );
            return Err(OracleError::DeserializationError.into());
        }

        Self::deserialize(&mut &data[..]).map_err(|err| {
            log!("Error: {} account deserialization failed: {err}", Self::name());
            OracleError::DeserializationError.into()
        })
    }

    #[track_caller]
    fn from_account_info(info: &AccountInfo) -> Result<Self, ProgramError> {
        let data = info.try_borrow_data()?;
        let account = Self::safe_deserialize(*data)?;

        Self::check_account_owner(info.owner)?;

        Ok(account)
    }
}

pub(crate) trait AccountSized: Account + BorshSize {
    #[track_caller]
    fn from_account_info_mut<'a, 'info>(
        info: &'a AccountInfo<'info>,
    ) -> Result<AccountSizedMut<'a, 'info, Self>, ProgramError> {
        let data = info.try_borrow_mut_data()?;
        let data = RefMut::map(data, |data| *data);

        let account = Self::safe_deserialize(&data)?;

        Self::check_account_owner(info.owner)?;

        Ok(AccountSizedMut { info, data, account })
    }
}

impl<T: Account + BorshSize> AccountSized for T {}

#[must_use = "Must call `.save()` to save account"]
pub(crate) struct AccountSizedMut<'a, 'info, T> {
    info: &'a AccountInfo<'info>,
    data: RefMut<'a, [u8]>,
    account: T,
}

impl<'a, 'info, T: AccountSized> AccountSizedMut<'a, 'info, T> {
    pub fn into_inner(self) -> T {
        self.account
    }

    pub fn realloc(
        &mut self,
        payer: &'a AccountInfo<'info>,
        system_program: &'a AccountInfo<'info>,
    ) -> ProgramResult {
        if T::IS_FIXED_SIZE {
            return Ok(());
        }

        let new_size = self.account.borsh_size();
        let current_size = self.data.len();

        if new_size <= current_size {
            return Ok(());
        }

        let rent = Rent::get()?;

        let current_rent = rent.minimum_balance(current_size);
        let new_rent = rent.minimum_balance(new_size);

        let rent_diff = new_rent.saturating_sub(current_rent);

        log!("Reallocating {} account data", T::name());

        // Reallocate the account and update the data reference.
        self.data = solana_utils::realloc_account_mut(self.info, new_size)?;

        if rent_diff > 0 {
            log!("Transferring {rent_diff} lamports for additional rent");

            // Transfer the additional rent required.
            cpi::sys::transfer(
                rent_diff,
                cpi::sys::Transfer { source: payer, destination: self.info, system_program },
                &[],
            )?;
        }

        Ok(())
    }

    pub fn save(mut self) -> Result<T, ProgramError> {
        if !T::IS_FIXED_SIZE {
            let size = self.borsh_size();

            if size > self.data.len() {
                log!("Error: {} account overflows allocation", T::name());
                return Err(OracleError::SerializationError.into());
            }
        }

        serialize_account(&mut *self.data, &self.account)?;

        Ok(self.account)
    }
}

impl<T> Deref for AccountSizedMut<'_, '_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.account
    }
}

impl<T> DerefMut for AccountSizedMut<'_, '_, T> {
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
            #[allow(unreachable_patterns)]
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
        let InitContext {
            account: account_info,
            payer,
            system_program,
            program_id,
            signers_seeds: signer_seeds,
        } = context;

        solana_utils::create_or_allocate_account(
            account_info,
            payer,
            system_program,
            self.space,
            program_id,
            signer_seeds,
        )?;

        serialize_account(account_info.try_borrow_mut_data()?.deref_mut(), &self.account)?;

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
    pub signers_seeds: &'a [&'b [&'c [u8]]],
}

fn serialize_account<W: io::Write, T: Account>(writer: W, account: &T) -> ProgramResult {
    borsh::to_writer(writer, account).map_err(|err| {
        log!("Error: {} serialization failed: {err}", T::name());
        OracleError::SerializationError.into()
    })
}
