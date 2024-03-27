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

pub const MIN_BOND: u64 = 1_000_000_000;
pub const GOVERNANCE_BOND: u64 = 1_000_000_000;

/// The number of seconds after an assertion is made in which it can be disputed (2 days).
pub const DISPUTE_WINDOW: i64 = 2 * 24 * 60 * 60;

solana_program::declare_id!("DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg");
