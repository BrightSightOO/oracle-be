use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, ShankAccount)]
pub struct Request {
    account_type: AccountType,

    /// Index of the request in the oracle.
    pub index: u64,

    /// Creator address.
    pub creator: Pubkey,

    /// Amount rewarded to the asserter/disputer on resolution.
    pub reward: u64,
    /// Reward mint.
    pub reward_mint: Pubkey,

    /// Unix timestamp for when a value can be asserted.
    pub timestamp: i64,

    /// Request state.
    pub state: RequestState,
    /// Value of the resolved request.
    pub value: u64,

    // Request data may have varying layouts when serialized. It is at the end
    // of the account to avoid interfering with GPA lookups.
    /// Request data.
    pub data: RequestData,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSize)]
#[repr(u8)]
pub enum RequestState {
    /// Request pending a proposal.
    Requested,
    /// Request with a asserted value awaiting resolution.
    Asserted,
    /// Request with a disputed value awaiting voting resolution.
    Disputed,
    /// Request with a resolved value.
    Resolved,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum RequestData {
    /// Yes/No request:
    /// - 0 = No
    /// - 1 = Yes
    YesNo {
        /// Question.
        question: String,
    },
}

impl Account for Request {
    const TYPE: AccountType = AccountType::Request;
}
