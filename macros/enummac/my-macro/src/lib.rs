extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(DebugMacro, attributes(my_custom_attr, variant_config, need_trim))]
pub fn derive_debug_macro(item: TokenStream) -> TokenStream {
    // println!("---------- input item ----------------");
    // println!("item: {:#?}", item);

    let input = parse_macro_input!(item as DeriveInput);

    let ident = &input.ident;

    println!("---------- parsed item {:#}----------------", ident);
    println!("parsed input: {:#?}", input);

    let func_name = format_ident!("debug_{}", ident);

    let expanded = quote! {
        pub fn #func_name() {
            println!("This is the auto-generated debug function for: {}", stringify!(#ident));
        }
    };

    TokenStream::from(expanded)
}
