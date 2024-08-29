use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::BorshSize;
use shank::ShankAccount;
use solana_program::clock::UnixTimestamp;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct VotingV1 {
    account_type: AccountType,

    /// The [`Request`]` this assertion is for.
    ///
    /// [`Request`]: crate::state::Request
    pub request: Pubkey,

    /// The address of the mint of the governance token required to vote.
    pub governance_mint: Pubkey,

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

impl Account for VotingV1 {
    const TYPE: AccountType = AccountType::VotingV1;
}

impl TryFrom<InitVoting> for (VotingV1, usize) {
    type Error = ProgramError;

    fn try_from(params: InitVoting) -> Result<(VotingV1, usize), Self::Error> {
        let InitVoting { request, governance_mint, start_timestamp, voting_window } = params;

        let end_timestamp = checked_add!(start_timestamp, i64::from(voting_window))?;

        let account = VotingV1 {
            account_type: VotingV1::TYPE,
            request,
            governance_mint,
            start_timestamp,
            end_timestamp,
            vote_count: 0,
            mode_value: 0,
            votes: BTreeMap::new(),
        };
        let space = account.borsh_size();

        Ok((account, space))
    }
}

pub(crate) struct InitVoting {
    pub request: Pubkey,
    pub governance_mint: Pubkey,

    pub start_timestamp: UnixTimestamp,
    pub voting_window: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_size() {
        let init = InitVoting {
            request: Pubkey::new_unique(),
            governance_mint: Pubkey::new_unique(),
            start_timestamp: 0,
            voting_window: 0,
        };

        let (mut account, expected) = <(VotingV1, usize)>::try_from(init).unwrap();
        let actual = account.try_to_vec().unwrap().len();

        assert_eq!(expected, actual);

        account.votes.insert(0, 10);
        account.votes.insert(1, 5);

        let expected = account.borsh_size();
        let actual = account.try_to_vec().unwrap().len();

        assert_eq!(expected, actual);
    }
}
