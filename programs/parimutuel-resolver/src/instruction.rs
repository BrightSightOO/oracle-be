use borsh::{BorshDeserialize, BorshSerialize};
use common::{BuildInstruction, VariantName};
use shank::{ShankContext, ShankInstruction};

#[rustfmt::skip::attributes(account)]
#[derive(
    Clone,
    VariantName,
    BuildInstruction,
    ShankContext,
    ShankInstruction,
    BorshDeserialize,
    BorshSerialize,
)]
pub enum ResolverInstruction {
    /// Creates a [`Resolver`].
    ///
    /// [`Resolver`]: crate::state::Resolver
    #[account(0, writable, name = "resolver", desc = "Resolver")]
    #[account(1, name = "market", desc = "Parimutuel market")]
    #[account(2, name = "request", desc = "Oracle request")]
    #[account(3, signer, writable, name = "payer", desc = "Payer")]
    #[account(4, name = "system_program", desc = "System program")]
    CreateResolver(CreateResolverArgs),

    /// Resolves the market based on the oracle request resolved value.
    #[account(0, name = "resolver", desc = "Resolver")]
    #[account(1, name = "market", desc = "Parimutuel market")]
    #[account(2, name = "request", desc = "Oracle request")]
    #[account(3, name = "parimutuel_program", desc = "Parimutuel program")]
    Resolve(ResolveArgs),
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum CreateResolverArgs {
    V1 {},
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub enum ResolveArgs {
    V1 {},
}
