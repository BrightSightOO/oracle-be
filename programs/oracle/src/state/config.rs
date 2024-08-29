use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct ConfigV1 {
    account_type: AccountType,

    /// Authority address.
    pub authority: Pubkey,

    /// The fee taken, in basis points, from the bond of the incorrect party in a dispute.
    pub bond_fee_bps: u16, // FIXME: Use Bps type.

    /// The duration of the dispute window in seconds.
    pub dispute_window: u32,
    /// The duration of the voting window in seconds.
    pub voting_window: u32,
    /// The duration of the arbitration window in seconds.
    pub arbitration_window: u32,
}

impl ConfigV1 {
    pub fn assert_authority(&self, authority: &Pubkey) -> Result<(), OracleError> {
        if !common::cmp_pubkeys(&self.authority, authority) {
            return Err(OracleError::ConfigAuthorityMismatch);
        }
        Ok(())
    }
}

impl Account for ConfigV1 {
    const TYPE: AccountType = AccountType::ConfigV1;
}

impl From<InitConfig> for (ConfigV1, usize) {
    fn from(params: InitConfig) -> (ConfigV1, usize) {
        let InitConfig {
            authority,
            bond_fee_bps,
            dispute_window,
            voting_window,
            arbitration_window,
        } = params;

        (
            ConfigV1 {
                account_type: ConfigV1::TYPE,
                authority,
                bond_fee_bps,
                dispute_window,
                voting_window,
                arbitration_window,
            },
            ConfigV1::SIZE,
        )
    }
}

pub(crate) struct InitConfig {
    pub authority: Pubkey,

    pub bond_fee_bps: u16,

    pub dispute_window: u32,
    pub voting_window: u32,
    pub arbitration_window: u32,
}
