use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::OracleError;

use super::{Account, AccountSized, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, ShankAccount)]
pub struct Voting {
    account_type: AccountType,

    /// The [`Request`]` this assertion is for.
    ///
    /// [`Request`]: crate::state::Request
    pub request: Pubkey,

    /// The Unix timestamp when voting started.
    pub start_timestamp: i64,
    /// The Unix timestamp when voting ends.
    pub end_timestamp: i64,

    /// The number of votes that have been added.
    pub vote_count: u64,
    /// The modal value, i.e. the value voted for the most.
    pub mode_value: u64,

    /// The votes for different values.
    pub votes: BTreeMap<u64, u64>,
}

impl Voting {
    const BASE_SIZE: usize =
        AccountType::SIZE       // account_type
        + Pubkey::SIZE          // request
        + i64::SIZE             // start_timestamp
        + i64::SIZE             // end_timestamp
        + u64::SIZE             // vote_count
        + u64::SIZE             // mode_value
        + u32::SIZE             // votes.len()
        ;
}

impl Account for Voting {
    const TYPE: AccountType = AccountType::Voting;
}

impl AccountSized for Voting {
    const IS_FIXED_SIZE: bool = false;

    fn serialized_size(&self) -> Option<usize> {
        self.votes.len().checked_mul(u64::SIZE)?.checked_add(Self::BASE_SIZE)
    }
}

impl TryFrom<InitVoting> for (Voting, usize) {
    type Error = OracleError;

    fn try_from(params: InitVoting) -> Result<(Voting, usize), Self::Error> {
        let InitVoting { request, start_timestamp } = params;

        let end_timestamp = start_timestamp
            .checked_add(crate::VOTING_WINDOW)
            .ok_or(OracleError::ArithmeticOverflow)?;

        Ok((
            Voting {
                account_type: Voting::TYPE,
                request,
                start_timestamp,
                end_timestamp,
                vote_count: 0,
                mode_value: 0,
                votes: BTreeMap::new(),
            },
            Voting::BASE_SIZE,
        ))
    }
}

pub(crate) struct InitVoting {
    pub request: Pubkey,
    pub start_timestamp: i64,
}
