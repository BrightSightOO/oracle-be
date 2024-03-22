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

// Export sdk types for downstream users with a different sdk version.
pub use solana_program;

pub const MIN_BOND: u64 = 1_000_000_000;
pub const GOVERNANCE_BOND: u64 = 1_000_000_000;

solana_program::declare_id!("AUCTiKuGUpoZXgbJguiq32uaL2uEViJg85VmSU2UMQHy");
