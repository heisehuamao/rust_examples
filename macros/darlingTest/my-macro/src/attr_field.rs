use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use darling::{FromDeriveInput, FromField, FromVariant};

// 1. Define parsing rules for struct fields
#[derive(Debug, FromField)]
#[darling(attributes(field_info, field_info_2))] // Looks for #[field_info(...)] on fields
struct MyFieldArgs {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    skip: bool,

    // You can now map fields specific to field_info_2 if you want
    #[darling(default)]
    another_param: Option<String>,
}

// 2. Define parsing rules for enum variants
#[derive(Debug, FromVariant)]
#[darling(attributes(variant_info, variant_info_2))] // Looks for #[variant_info(...)] on variants
struct MyVariantArgs {
    ident: syn::Ident,

    #[darling(default)]
    rename: Option<String>,

    // You can now map variants specific to variant_info_2 if you want
    #[darling(default)]
    custom_id: Option<i32>,
}

// 3. Define the main receiver for the entire Struct or Enum
#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_any, enum_any))]
struct MyContainerArgs {
    ident: syn::Ident,

    // Automatically populated if it's a struct
    data: darling::ast::Data<MyVariantArgs, MyFieldArgs>,
}

pub fn my_attri_field_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Treat the input token stream as a standard DeriveInput item
    let mut raw_input = parse_macro_input!(input as DeriveInput);

    // Parse everything using darling
    let container = match MyContainerArgs::from_derive_input(&raw_input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    // Inspect what we parsed based on whether it's a struct or an enum
    match container.data {
        darling::ast::Data::Struct(fields) => {
            println!("Parsing fields for struct: {}", container.ident);
            for field in fields.fields {
                if let Some(ident) = field.ident {
                    println!(
                        " -> Field: {}, Is Skipped? {}, another_param: {:?}",
                        ident, field.skip, field.another_param
                    );
                }
            }
        }
        darling::ast::Data::Enum(variants) => {
            println!("Parsing variants for enum: {}", container.ident);
            for variant in variants {
                println!(
                    " -> Variant: {}, Renamed to: {:?}, custom_id: {:?}",
                    variant.ident, variant.rename, variant.custom_id
                );
            }
        }
    }

    // 3. STRIP OUT the helper attributes so the compiler doesn't complain
    clean_helper_attributes(&mut raw_input);

    // Since it's an attribute macro, we must return the original item
    // (or a modified version of it) so it remains valid Rust code.
    TokenStream::from(quote! {
        #raw_input
    })
}

// Helper function to remove the attributes from the AST
fn clean_helper_attributes(input: &mut DeriveInput) {
    let is_field_attr = |attr: &syn::Attribute| {
        attr.path().is_ident("field_info") || attr.path().is_ident("field_info_2")
    };
    let is_variant_attr = |attr: &syn::Attribute| {
        attr.path().is_ident("variant_info") || attr.path().is_ident("variant_info_2")
    };
    match &mut input.data {
        syn::Data::Struct(data_struct) => {
            for field in &mut data_struct.fields {
                field.attrs.retain(|attr| !is_field_attr(attr));
            }
        }
        syn::Data::Enum(data_enum) => {
            for variant in &mut data_enum.variants {
                variant.attrs.retain(|attr| !is_variant_attr(attr));
                // Also clean fields inside tuple/struct enum variants if they have them
                for field in &mut variant.fields {
                    field.attrs.retain(|attr| !is_field_attr(attr));
                }
            }
        }
        syn::Data::Union(data_union) => {
            for field in &mut data_union.fields.named {
                field.attrs.retain(|attr| !is_field_attr(attr));
            }
        }
    }
}
