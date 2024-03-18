use proc_macro2::{Span, TokenStream};
use proc_macro2_diagnostics::SpanDiagnosticExt;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput};

use crate::Result;

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let msg = match &input.data {
        Data::Enum(data) => return expand_enum(&input, data),
        Data::Struct(..) => "this trait cannot be derived for structs",
        Data::Union(..) => "this trait cannot be derived for unions",
    };
    Err(Span::call_site().error(msg))
}

fn expand_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    let ty = &input.ident;

    let arms = data.variants.iter().map(|v| {
        let variant = &v.ident;
        let variant_name = variant.to_string();

        quote!(#ty::#variant { .. } => #variant_name)
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::common::VariantName for #ty #ty_generics #where_clause {
            fn variant_name(&self) -> &'static str {
                match self {
                    #(#arms,)*
                }
            }
        }
    })
}
