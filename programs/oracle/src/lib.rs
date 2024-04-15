#![deny(clippy::disallowed_macros, clippy::disallowed_methods, clippy::disallowed_types)]

#[macro_use]
mod macros;
mod cpi;
mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod pda;
pub mod processor;
pub mod state;

/// The duration of the dispute window (15 secs).
pub const DISPUTE_WINDOW: i64 = 15; // 2 * 24 * 60 * 60;

/// The duration of the voting window (5 mins).
pub const VOTING_WINDOW: i64 = 5 * 60; // 1 * 24 * 60 * 60;

solana_program::declare_id!("DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg");
