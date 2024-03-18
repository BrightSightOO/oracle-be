#[allow(clippy::disallowed_macros)]
mod generated;
mod hooked;

pub use crate::generated::programs::ORACLE_ID as ID;
pub use crate::generated::*;
pub use crate::hooked::prelude::*;
