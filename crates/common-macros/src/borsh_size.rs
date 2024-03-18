use proc_macro2::{Ident, Span, TokenStream};
use proc_macro2_diagnostics::SpanDiagnosticExt;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Generics};

use crate::Result;

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let params = Parameters { ident: input.ident, generics: input.generics };
    let msg = match input.data {
        Data::Enum(data) => return expand_enum(data, params),
        Data::Struct(data) => return expand_struct(data, params),
        Data::Union(..) => "this trait cannot be derived for unions",
    };
    Err(Span::call_site().error(msg))
}

fn expand_enum(data: DataEnum, params: Parameters) -> Result<TokenStream> {
    if data.variants.is_empty() {
        return Err(Span::call_site().error("this trait does not support enums with no variants"));
    }

    if data.variants.len() > 255 {
        return Err(Span::call_site()
            .error("this trait does not support enums with more than 255 variants")
            .span_help(data.variants[255].ident.span(), "this is the 256th variant"));
    }

    let ident = &params.ident;
    let (impl_generics, ty_generics, where_clause) = params.generics.split_for_impl();

    let variant_sizes = data.variants.into_iter().map(|v| expand_fields(v.fields));

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::common::BorshSize for #ident #ty_generics #where_clause {
            const SIZE: usize = 1 + {
                let mut max_variant = 0;
                #({
                    const VARIANT_SIZE: usize = #variant_sizes;
                    if VARIANT_SIZE > max_variant {
                        max_variant = VARIANT_SIZE;
                    }
                })*
                max_variant
            };
        }
    })
}

fn expand_struct(data: DataStruct, params: Parameters) -> Result<TokenStream> {
    let ident = &params.ident;
    let (impl_generics, ty_generics, where_clause) = params.generics.split_for_impl();

    let size = expand_fields(data.fields);

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::common::BorshSize for #ident #ty_generics #where_clause {
            const SIZE: usize = #size;
        }
    })
}

fn expand_fields(fields: Fields) -> TokenStream {
    let mut sizes = fields.into_iter().map(|mut field| {
        // Clear field attributes so they are not included in the span.
        field.attrs.clear();

        let field_ty = &field.ty;
        let span = field.span().resolved_at(Span::call_site());

        quote_spanned! { span => <#field_ty as ::common::BorshSize>::SIZE }
    });

    if let Some(first) = sizes.next() {
        quote! { #first #(+ #sizes)* }
    } else {
        quote! { 0 }
    }
}

struct Parameters {
    ident: Ident,
    generics: Generics,
}
