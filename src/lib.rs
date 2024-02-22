mod convert;

use proc_macro::TokenStream;
use convert_case::Case;
use quote::ToTokens;
use crate::convert::Convertible;

fn convert(input: proc_macro2::TokenStream, case: Case) -> syn::Result<proc_macro2::TokenStream> {
    let input: Convertible = syn::parse2(input)?;
    input.new_with_case(case).map(Convertible::into_token_stream)
}

fn convert_impl(input: TokenStream, case: Case) -> TokenStream {
    let result = convert(input.into(), case).unwrap_or_else(|e| e.into_compile_error());
    result.into()
}

#[proc_macro]
pub fn uppercase(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::Upper)
}
#[proc_macro]
pub fn lowercase(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::Lower)
}
#[proc_macro]
pub fn title_case(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::Title)
}
#[proc_macro]
pub fn camel_case(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::Camel)
}
#[proc_macro]
pub fn pascal_case(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::Pascal)
}
#[proc_macro]
pub fn snake_case(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::Snake)
}
#[proc_macro]
pub fn upper_snake_case(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::UpperSnake)
}
#[proc_macro]
pub fn kebab_case(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::Kebab)
}
#[proc_macro]
pub fn cobol_case(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::Cobol)
}
#[proc_macro]
pub fn train_case(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::Train)
}
#[proc_macro]
pub fn flat_case(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::Flat)
}
#[proc_macro]
pub fn upper_flat_case(input: TokenStream) -> TokenStream {
    convert_impl(input, Case::UpperFlat)
}