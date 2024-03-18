use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::{
    parse_quote, Attribute, Data, DataEnum, DeriveInput, Error, Fields, FieldsUnnamed, LitInt,
    LitStr, Path, Result, Token, Type, Variant,
};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let msg = match &input.data {
        Data::Enum(data) => return expand_enum(&input, data),
        Data::Struct(..) => "this trait cannot be derived for structs",
        Data::Union(..) => "this trait cannot be derived for unions",
    };
    Err(Error::new(Span::call_site(), msg))
}

#[derive(Debug)]
struct Instruction {
    name: Ident,
    args: Type,
    accounts: Vec<Account>,
}

#[derive(Debug)]
struct Account {
    name: LitStr,
    signer: bool,
    writable: bool,
    optional: bool,
    desc: Option<String>,
}

#[derive(Debug)]
struct PartialAccount {
    position: Option<LitInt>,
    name: Option<LitStr>,
    signer: bool,
    writable: bool,
    optional: bool,
    desc: Option<String>,
}

fn expand_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    let ixs = parse_instructions(data)?;

    let borsh: Path = parse_quote! { ::borsh };
    let solana_program: Path = parse_quote! { ::solana_program };

    let enum_name = &input.ident;
    let trait_name = Ident::new(&format!("Build{enum_name}"), Span::call_site());

    let impls = ixs.iter().map(|ix| {
        let ix_name = &ix.name;
        let args_ty = &ix.args;

        let accounts = ix.accounts.iter().map(|account| {
            let account_name = Ident::new(&account.name.value(), account.name.span());
            let docs = match account.desc.as_ref() {
                Some(desc) => quote! { #[doc = #desc] },
                None => quote! {},
            };
            if account.optional {
                quote! {
                    #docs
                    pub #account_name: Option<#solana_program::pubkey::Pubkey>,
                }
            } else {
                quote! {
                    #docs
                    pub #account_name: #solana_program::pubkey::Pubkey,
                }
            }
        });

        let metas = ix.accounts.iter().map(|account| {
            let account_name = Ident::new(&account.name.value(), account.name.span());

            let pubkey = if account.optional {
                quote! { self.#account_name.unwrap_or(crate::ID) }
            } else {
                quote! { self.#account_name }
            };

            let new_fn = if account.writable {
                quote! { new }
            } else {
                quote! { new_readonly }
            };

            let is_signer = account.signer;

            quote! {
                #solana_program::instruction::AccountMeta::#new_fn(#pubkey, #is_signer)
            }
        });

        quote! {
            pub use super::#args_ty;

            #[derive(Clone, Debug)]
            pub struct #ix_name {
                #(#accounts)*
            }

            impl #trait_name for #ix_name {
                type Args = #args_ty;

                fn account_metas(self) -> Vec<#solana_program::instruction::AccountMeta> {
                    vec![
                        #(#metas,)*
                    ]
                }

                fn instruction(self, args: #args_ty) -> BorshResult<#solana_program::instruction::Instruction> {
                    let args = #enum_name::#ix_name(args);
                    let data = #borsh::to_vec(&args)?;
                    Ok(#solana_program::instruction::Instruction {
                        program_id: crate::ID,
                        accounts: self.account_metas(),
                        data,
                    })
                }
            }
        }
    });

    Ok(quote! {
        pub mod build {
            use super::*;

            use ::std::io::Result as BorshResult;

            pub trait #trait_name {
                type Args: Sized;
                fn account_metas(self) -> Vec<#solana_program::instruction::AccountMeta>;
                fn instruction(self, args: Self::Args) -> BorshResult<#solana_program::instruction::Instruction>;
            }

            #(#impls)*
        }
    })
}

fn parse_instructions(data: &DataEnum) -> Result<Vec<Instruction>> {
    fn parse_instruction(v: &Variant) -> Result<Instruction> {
        let args = parse_instruction_args(&v.fields)?;
        let accounts = parse_instruction_accounts(&v.attrs)?;

        Ok(Instruction { name: v.ident.clone(), args, accounts })
    }

    let mut iter = data.variants.iter();

    let mut ixs = Vec::new();

    let mut errs = loop {
        let v = match iter.next() {
            Some(v) => v,
            None => return Ok(ixs),
        };

        match parse_instruction(v) {
            Ok(ix) => ixs.push(ix),
            Err(err) => break err,
        }
    };

    for v in iter {
        if let Err(err) = parse_instruction(v) {
            errs.combine(err);
        }
    }

    Err(errs)
}

fn parse_instruction_args(fields: &Fields) -> Result<Type> {
    let unnamed = match fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
        _ => return Err(Error::new_spanned(fields, "instruction variants must be tuple style")),
    };

    let mut iter = unnamed.iter();

    let args = match iter.next() {
        Some(args) => args,
        None => return Err(Error::new_spanned(unnamed, "instruction must have an argument type")),
    };

    // Check for extraneous arguments.
    let mut span = match iter.next() {
        None => return Ok(args.ty.clone()),
        Some(extra) => extra.span(),
    };

    for extra in iter {
        match span.join(extra.span()) {
            Some(joined) => span = joined,
            None => break,
        }
    }

    Err(Error::new(span, "instruction can only have a single argument type"))
}

fn parse_instruction_accounts(attrs: &[Attribute]) -> Result<Vec<Account>> {
    let mut accounts = Vec::<Account>::new();

    for attr in attrs {
        if !attr.path().is_ident("account") {
            continue;
        }

        let PartialAccount { position, name, signer, writable, optional, desc } =
            attr.parse_args_with(parse_account_attr)?;

        let name = match name {
            Some(name) => name,
            None => return Err(Error::new_spanned(&attr.meta, "missing account name")),
        };

        if let Some(pos) = position {
            let value = match pos.base10_parse::<usize>() {
                Ok(value) => value,
                _ => return Err(Error::new_spanned(pos, "invalid account index")),
            };

            let expected = accounts.len();
            if value != expected {
                return Err(Error::new_spanned(
                    pos,
                    format_args!("incorrect account index (expected {expected})"),
                ));
            }
        }

        accounts.push(Account { name, signer, writable, optional, desc });
    }

    Ok(accounts)
}

fn parse_account_attr(input: ParseStream) -> Result<PartialAccount> {
    let position = if input.peek(LitInt) {
        let pos: LitInt = input.parse()?;
        let _: Token![,] = input.parse()?;

        if !pos.suffix().is_empty() {
            return Err(Error::new_spanned(pos, "invalid account position"));
        }

        Some(pos)
    } else {
        None
    };

    let mut name = None::<LitStr>;

    let mut writable = false;
    let mut optional = false;
    let mut signer = false;
    let mut desc = None::<String>;

    loop {
        let ident = if input.peek(Ident::peek_any) {
            Ident::parse_any(input)?
        } else if input.is_empty() {
            return Err(input.error("expected nested attribute"));
        } else {
            return Err(input.error("unexpected token in nested attribute, expected ident"));
        };

        if ident == "name" {
            if name.is_some() {
                return Err(input.error("duplicate attribute"));
            }

            let _: Token![=] = input.parse()?;
            let s: LitStr = input.parse()?;

            name = Some(s);
        } else if ident == "signer" {
            if signer {
                return Err(input.error("duplicate property"));
            }

            signer = true;
        } else if ident == "writable" {
            if writable {
                return Err(input.error("duplicate property"));
            }

            writable = true;
        } else if ident == "optional" {
            if optional {
                return Err(input.error("duplicate property"));
            }

            optional = true;
        } else if ident == "desc" {
            if desc.is_some() {
                return Err(input.error("duplicate property"));
            }

            let _: Token![=] = input.parse()?;
            let s: LitStr = input.parse()?;

            desc = Some(s.value());
        } else {
            return Err(Error::new_spanned(ident, "unsupported account property"));
        }

        if input.is_empty() {
            break;
        }
        input.parse::<Token![,]>()?;
        if input.is_empty() {
            break;
        }
    }

    Ok(PartialAccount { position, name, signer, writable, optional, desc })
}
