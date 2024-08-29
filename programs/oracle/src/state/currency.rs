use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::{BorshSize, BorshSizeProperties};
use shank::ShankAccount;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;
use crate::pda;
use crate::utils::Bounds;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct CurrencyV1 {
    account_type: AccountType,

    /// The config address.
    pub config: Pubkey,
    /// The mint address.
    pub mint: Pubkey,

    /// The valid reward range when creating a request.
    pub reward_range: Bounds,
    /// The valid bond range when creating an assertion.
    pub bond_range: Bounds,
}

impl CurrencyV1 {
    pub fn assert_pda(&self, currency: &Pubkey) -> Result<u8, ProgramError> {
        pda::currency::assert_pda(currency, &self.config, &self.mint)
    }

    pub fn assert_config(&self, config: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(&self.config, config) {
            return Err(OracleError::ConfigMismatch);
        }
        Ok(())
    }

    pub fn assert_mint(&self, mint: &Pubkey) -> Result<(), OracleError> {
        if !solana_utils::pubkeys_eq(&self.mint, mint) {
            return Err(OracleError::CurrencyMintMismatch);
        }
        Ok(())
    }
}

impl Account for CurrencyV1 {
    const TYPE: AccountType = AccountType::CurrencyV1;
}

impl From<InitCurrency> for (CurrencyV1, usize) {
    fn from(params: InitCurrency) -> (CurrencyV1, usize) {
        let InitCurrency { config, mint, reward_range, bond_range } = params;

        (
            CurrencyV1 { account_type: CurrencyV1::TYPE, config, mint, reward_range, bond_range },
            CurrencyV1::FIXED_SIZE,
        )
    }
}

pub(crate) struct InitCurrency {
    pub config: Pubkey,
    pub mint: Pubkey,
    pub reward_range: Bounds,
    pub bond_range: Bounds,
}
