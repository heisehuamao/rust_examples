#![allow(warnings)]

use darling::{FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

// 1. Define the struct that models your expected macro arguments.
//    Darling will automatically parse key-value pairs, nested lists, and flags.
#[derive(Debug, FromMeta)]
struct MyMacroArgs {
    // A mandatory string argument: name = "something"
    name: String,

    // An optional integer argument: timeout = 30
    // Wrap in Option so it doesn't fail parsing if omitted.
    timeout: Option<u64>,

    // A boolean flag. If the user writes `verbose`, this becomes true.
    #[darling(default)]
    verbose: bool,
}

pub fn my_custom_attribute_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(darling::Error::from(e).write_errors()),
    };
    println!("attr_args is : {:#?}", attr_args);

    let macro_args = match MyMacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let v_input = parse_macro_input!(input as syn::Item);

    let expanded: proc_macro2::TokenStream = match v_input {
        syn::Item::Struct(s) => {
            let st_name = &s.ident;
            if macro_args.verbose {
                println!(
                    "Macro target '{}' is generating with verbose logging.",
                    st_name
                );
            }

            println!("struct macro args is: {:#?}", macro_args);

            quote! { #s}
        }
        syn::Item::Fn(f) => {
            let fn_name = &f.sig.ident;

            // You now have typed access to macro parameters!
            if macro_args.verbose {
                println!(
                    "Macro target '{}' is generating with verbose logging.",
                    fn_name
                );
            }
            println!("function macro args is: {:#?}", macro_args);

            // 4. Output the generated token stream
            quote! {
                #f
            }
        }
        _ => panic!("Unsupported item type"),
    };

    TokenStream::from(expanded)
}
