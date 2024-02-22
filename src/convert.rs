use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{LitByteStr, LitStr};
use syn::parse::{Parse, ParseStream};

pub enum Convertible {
    Ident(Ident),
    Str(LitStr),
    ByteStr(LitByteStr),
}

impl Convertible {
    pub fn new_with_case(&self, case: Case) -> syn::Result<Self> {
        Ok(match self {
            Self::Ident(ident) => {
                let new_ident = Ident::new(&ident.to_string().to_case(case), ident.span());
                Self::Ident(new_ident)
            },
            Self::Str(str) => {
                let new_str = LitStr::new(&str.value().to_case(case), str.span());
                Self::Str(new_str)
            },
            Self::ByteStr(byte_str) => {
                let string = String::from_utf8(byte_str.value()).map_err(|e| syn::Error::new(byte_str.span(), e))?;
                let new_byte_str = LitByteStr::new(string.as_bytes(), byte_str.span());
                Self::ByteStr(new_byte_str)
            }
        })
    }
}

impl Parse for Convertible {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Ident) {
            return input.parse().map(Self::Ident)
        }
        if input.peek(syn::LitStr) {
            return input.parse().map(Self::Str)
        }
        if input.peek(syn::LitByteStr) {
            return input.parse().map(Self::ByteStr)
        }
        Err(syn::Error::new(input.span(), "Must be an identifier or a string literal"))
    }
}

impl ToTokens for Convertible {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Str(str) => str.to_tokens(tokens),
            Self::ByteStr(byte_str) => byte_str.to_tokens(tokens)
        }
    }
}