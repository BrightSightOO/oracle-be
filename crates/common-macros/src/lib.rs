use proc_macro::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use syn::{parse_macro_input, DeriveInput, Error};

mod borsh_size;
mod build_instruction;
mod variant_name;

pub(crate) type Result<T, E = Diagnostic> = std::result::Result<T, E>;

#[proc_macro_derive(VariantName)]
pub fn variant_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    variant_name::expand(input).unwrap_or_else(Diagnostic::emit_as_item_tokens).into()
}

#[proc_macro_derive(BuildInstruction, attributes(account))]
pub fn build_instruction(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    build_instruction::expand(input).unwrap_or_else(Error::into_compile_error).into()
}

#[proc_macro_derive(BorshSize)]
pub fn borsh_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    borsh_size::expand(input).unwrap_or_else(Diagnostic::emit_as_item_tokens).into()
}
