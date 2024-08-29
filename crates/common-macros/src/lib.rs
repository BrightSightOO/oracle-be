use proc_macro::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use syn::{parse_macro_input, DeriveInput};

mod borsh_size;
mod variant_name;

pub(crate) type Result<T, E = Diagnostic> = std::result::Result<T, E>;

#[proc_macro_derive(VariantName)]
pub fn variant_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    variant_name::expand(input).unwrap_or_else(Diagnostic::emit_as_item_tokens).into()
}

#[proc_macro_derive(BorshSize)]
pub fn borsh_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    borsh_size::expand(input).unwrap_or_else(Diagnostic::emit_as_item_tokens).into()
}
