use proc_macro::TokenStream;

use darling::{FromDeriveInput, FromField, FromVariant};
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

// 1. For individual struct/enum fields
#[derive(Debug, FromField)]
#[darling(attributes(my_trait))]
pub struct MyFieldOpts {
    pub ident: Option<syn::Ident>, // None for tuple fields, Some for named fields
    pub ty: syn::Type,             // The type of the field (e.g., String, i32)

    /// Custom attribute: #[my_trait(rename = "new_name")]
    #[darling(default)]
    pub rename: Option<String>,
}

// 2. For enum variants
#[derive(Debug, FromVariant)]
#[darling(attributes(my_trait))]
pub struct MyVariantOpts {
    pub ident: syn::Ident,

    /// Custom attribute: #[my_trait(skip)]
    pub skip: darling::util::Flag,

    pub version: Option<i32>,
}

// 1. Define a struct to capture the attributes applied to the type
#[derive(Debug, FromDeriveInput)]
// This specifies the helper attribute name we are looking for: #[my_trait(...)]
#[darling(attributes(my_trait))]
struct MyTraitOpts {
    ident: syn::Ident, // Automatically captures the annotated struct/enum name

    // Automatically populated based on data(fields = "...", variants = "...")
    pub data: darling::ast::Data<MyVariantOpts, MyFieldOpts>,

    // Parsed from #[my_trait(msg = "Hello")]
    #[darling(map = "uppercase_string")]
    msg: String,

    // Parsed from #[my_trait(count = 5)] (Optional, defaults to 1 if omitted)
    #[darling(default = "default_count")]
    count: usize,
}

fn uppercase_string(s: String) -> String {
    s.to_uppercase()
}

fn default_count() -> usize {
    1
}

pub fn derive_attr_impl(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Use darling to parse helper attributes directly from the DeriveInput
    let opts = match MyTraitOpts::from_derive_input(&input) {
        Ok(val) => val,
        Err(err) => {
            // Return any parsing errors back to the compiler
            return TokenStream::from(err.write_errors());
        }
    };

    println!("opts data is : {:#?}", opts.data);

    let struct_name = opts.ident;
    let msg = opts.msg;
    let count = opts.count;

    // Generate the implementation code
    let expanded = quote! {
        impl #struct_name {
            pub fn print_message(&self) {
                for _ in 0..#count {
                    println!("{}", #msg);
                }
            }
        }
    };

    TokenStream::from(expanded)
}
