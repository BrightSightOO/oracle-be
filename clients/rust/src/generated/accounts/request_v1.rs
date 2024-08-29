//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::generated::types::{AccountType, RequestData, RequestState};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequestV1 {
    pub account_type: AccountType,
    pub index: u64,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
    pub config: Pubkey,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
    pub creator: Pubkey,
    pub reward: u64,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
    pub reward_mint: Pubkey,
    pub bond: u64,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
    pub bond_mint: Pubkey,
    pub assertion_timestamp: i64,
    pub resolve_timestamp: i64,
    pub state: RequestState,
    pub value: u64,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
    pub arbitrator: Pubkey,
    pub data: RequestData,
}

impl RequestV1 {
    /// Prefix values used to generate a PDA for this account.
    ///
    /// Values are positional and appear in the following order:
    ///
    ///   0. `RequestV1::PREFIX`
    ///   1. index (`u64`)
    pub const PREFIX: &'static [u8] = "request".as_bytes();

    pub fn create_pda(
        index: u64,
        bump: u8,
    ) -> Result<solana_program::pubkey::Pubkey, solana_program::pubkey::PubkeyError> {
        solana_program::pubkey::Pubkey::create_program_address(
            &["request".as_bytes(), index.to_string().as_ref(), &[bump]],
            &crate::OPTIMISTIC_ORACLE_ID,
        )
    }

    pub fn find_pda(index: u64) -> (solana_program::pubkey::Pubkey, u8) {
        solana_program::pubkey::Pubkey::find_program_address(
            &["request".as_bytes(), index.to_string().as_ref()],
            &crate::OPTIMISTIC_ORACLE_ID,
        )
    }

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for RequestV1 {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for RequestV1 {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for RequestV1 {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for RequestV1 {
    fn owner() -> Pubkey {
        crate::OPTIMISTIC_ORACLE_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for RequestV1 {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for RequestV1 {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
