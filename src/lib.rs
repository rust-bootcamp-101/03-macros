mod auto_debug;
mod auto_deref;
mod enum_from;
mod enum_from_darling;

use auto_debug::process_auto_debug;
use auto_deref::process_auto_deref;
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

// attributes(deref)，表示 使用#[deref(xx=xx)]
#[proc_macro_derive(AutoDeref, attributes(deref))]
pub fn derive_auto_deref(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_auto_deref(input)
}

// attributes(debug) 获取字段属性 如在字段上打上了 #[debug(xxx)]，获取到 xxx
#[proc_macro_derive(AutoDebug, attributes(debug))]
pub fn derive_auto_debug(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_auto_debug(input)
}
