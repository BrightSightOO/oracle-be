use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct Vote {
    account_type: AccountType,

    /// The address of the [`Stake`] the votes represent.
    ///
    /// [`Stake`]: crate::state::Stake
    pub stake: Pubkey,

    /// The value voted for.
    pub value: u64,
    /// The amount of votes.
    pub votes: u64,
}

impl Account for Vote {
    const TYPE: AccountType = AccountType::Vote;
}

impl From<InitVote> for (Vote, usize) {
    fn from(params: InitVote) -> (Vote, usize) {
        let InitVote { stake, value, votes } = params;

        (Vote { account_type: Vote::TYPE, stake, value, votes }, Vote::SIZE)
    }
}

pub(crate) struct InitVote {
    pub stake: Pubkey,
    pub value: u64,
    pub votes: u64,
}
