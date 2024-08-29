use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct VoteV1 {
    account_type: AccountType,

    /// The address of the voting account.
    pub voting: Pubkey,
    /// The address of the stake the votes represent.
    pub stake: Pubkey,

    /// The value voted for.
    pub value: u64,
    /// The amount of votes.
    pub votes: u64,
}

impl Account for VoteV1 {
    const TYPE: AccountType = AccountType::VoteV1;
}

impl From<InitVote> for (VoteV1, usize) {
    fn from(params: InitVote) -> (VoteV1, usize) {
        let InitVote { voting, stake, value, votes } = params;

        (VoteV1 { account_type: VoteV1::TYPE, voting, stake, value, votes }, VoteV1::SIZE)
    }
}

pub(crate) struct InitVote {
    pub voting: Pubkey,
    pub stake: Pubkey,
    pub value: u64,
    pub votes: u64,
}
