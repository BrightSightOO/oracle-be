use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::clock::UnixTimestamp;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountSized, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, ShankAccount)]
pub struct VotingV1 {
    account_type: AccountType,

    /// The [`Request`]` this assertion is for.
    ///
    /// [`Request`]: crate::state::Request
    pub request: Pubkey,

    /// The Unix timestamp when voting started.
    pub start_timestamp: UnixTimestamp,
    /// The Unix timestamp when voting ends.
    pub end_timestamp: UnixTimestamp,

    /// The number of votes that have been added.
    pub vote_count: u64,
    /// The modal value, i.e. the value voted for the most.
    pub mode_value: u64,

    /// The votes for different values.
    pub votes: BTreeMap<u64, u64>,
}

impl VotingV1 {
    const BASE_SIZE: usize =
        AccountType::SIZE       // account_type
        + Pubkey::SIZE          // request
        + UnixTimestamp::SIZE   // start_timestamp
        + UnixTimestamp::SIZE   // end_timestamp
        + u64::SIZE             // vote_count
        + u64::SIZE             // mode_value
        + u32::SIZE             // votes.len()
        ;
}

impl Account for VotingV1 {
    const TYPE: AccountType = AccountType::VotingV1;
}

impl AccountSized for VotingV1 {
    const IS_FIXED_SIZE: bool = false;

    fn serialized_size(&self) -> Option<usize> {
        self.votes.len().checked_mul(u64::SIZE + u64::SIZE)?.checked_add(Self::BASE_SIZE)
    }
}

impl TryFrom<InitVoting> for (VotingV1, usize) {
    type Error = ProgramError;

    fn try_from(params: InitVoting) -> Result<(VotingV1, usize), Self::Error> {
        let InitVoting { request, start_timestamp, voting_window } = params;

        let end_timestamp = checked_add!(start_timestamp, i64::from(voting_window))?;

        Ok((
            VotingV1 {
                account_type: VotingV1::TYPE,
                request,
                start_timestamp,
                end_timestamp,
                vote_count: 0,
                mode_value: 0,
                votes: BTreeMap::new(),
            },
            VotingV1::BASE_SIZE,
        ))
    }
}

pub(crate) struct InitVoting {
    pub request: Pubkey,
    pub start_timestamp: UnixTimestamp,

    pub voting_window: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_size() {
        let init =
            InitVoting { request: Pubkey::new_unique(), start_timestamp: 0, voting_window: 0 };

        let (mut account, expected) = <(VotingV1, usize)>::try_from(init).unwrap();
        let actual = common_test::serialized_len(&account).unwrap();

        assert_eq!(expected, actual);

        account.votes.insert(0, 10);
        account.votes.insert(1, 5);

        let expected = account.serialized_size().unwrap();
        let actual = common_test::serialized_len(&account).unwrap();

        assert_eq!(expected, actual);
    }
}
