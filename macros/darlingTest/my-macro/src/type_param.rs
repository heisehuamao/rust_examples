use proc_macro::TokenStream;

use darling::util::Flag; // 👈 Import the Flag utility
use darling::{FromDeriveInput, FromMeta, FromTypeParam};
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

// 1. Define parsing rules for individual Type Parameters
#[derive(Debug, FromTypeParam)]
// Control which attributes get forwarded from the type parameter to `attrs`
#[darling(forward_attrs(doc), attributes(my_type_attr))]
struct MyTypeParamArgs {
    /// The identifier of the passed-in type param (e.g., `T`)
    ident: syn::Ident,

    /// The bounds applied to the type param (e.g., `: Clone + Send`)
    bounds: Vec<syn::TypeParamBound>,

    /// The default type of the parameter, if one exists (e.g., `= String`)
    default: Option<syn::Type>,

    /// The forwarded attributes collected from the type param
    attrs: Vec<syn::Attribute>,

    // Darling flattens or extracts the meta items directly into your metadata field
    // Note: Use Option if the attribute itself is optional on the type param
    // #[darling(default, rename = "my_type_attr")]
    // meta_args: Option<MyTypeAttrArgs>,
    a: Option<bool>,

    b: Option<bool>,

    c: Flag,
}

// 2. Define the main receiver for the struct/enum to capture its generics
#[derive(Debug, FromDeriveInput)]
struct MyContainerArgs {
    ident: syn::Ident,

    // darling will automatically parse the type parameters into your struct
    generics: darling::ast::Generics<darling::ast::GenericParam<MyTypeParamArgs>>,
}

pub fn process_type_params_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut raw_input = parse_macro_input!(input as DeriveInput);

    let container = match MyContainerArgs::from_derive_input(&raw_input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    println!("===================");
    println!("container ident is {:?}", container.ident);

    // 3. Iterate over the type parameters.
    //    darling::ast::Generics separates them automatically.
    for param in container.generics.type_params() {
        println!(" -> Parameter Name: {}", param.ident);
        println!("    Number of Bounds: {}", param.bounds.len());
        println!("    Has Default? {}", param.default.is_some());
        println!("    attrs: {:?}", param.attrs);

        println!(" -> flag a: {:?}", param.a);
        println!(" -> flag b: {:?}", param.b);
        println!(" -> flag c: {:?}", param.c);
    }

    clean_helper_attributes(&mut raw_input);

    TokenStream::from(quote! {
        #raw_input
    })
}

fn clean_helper_attributes(input: &mut DeriveInput) {
    // Clean attributes attached to the Type Parameters themselves
    for param in &mut input.generics.params {
        if let syn::GenericParam::Type(type_param) = param {
            type_param
                .attrs
                .retain(|attr| !attr.path().is_ident("my_type_attr"));
        }
    }
}
