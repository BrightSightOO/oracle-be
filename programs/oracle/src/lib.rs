#![deny(clippy::disallowed_macros, clippy::disallowed_methods, clippy::disallowed_types)]

#[macro_use]
mod macros;
mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod pda;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
security_txt::security_txt! {
    // Required fields.
    name: "Optimistic Oracle",
    project_url: "https://github.com/BrightSightOO",
    contacts: "email:james@hedgehog.markets",
    policy: "https://github.com/BrightSightOO/oracle-be/security",

    // Optional fields.
    source_code: "https://github.com/BrightSightOO/oracle-be",
}

#[cfg(not(feature = "no-entrypoint"))]
include_idl::include_idl!(concat!(env!("OUT_DIR"), "/solana.idl.zip"));

solana_program::declare_id!("DVM2j1a1AJ9hZuEXyMxA5vusKgMR2FcKJyCf3QE5R2ge");
