mod enum_from;
mod enum_from_darling;

use enum_from::process_enum_from;
use enum_from_darling::process_enum_from_darling;
use proc_macro::TokenStream;

// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)] // 函数名不重要，重要的是宏的名字，如这个宏的名字叫 EnumFrom
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_from(input)
}

// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFromDarling)] // 函数名不重要，重要的是宏的名字，如这个宏的名字叫 EnumFromDarling
pub fn derive_enum_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_enum_from_darling(input)
}
