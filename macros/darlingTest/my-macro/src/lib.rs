extern crate proc_macro;

use proc_macro::TokenStream;

mod attr_field;
mod attribute_macro;
mod type_param;

use crate::{
    attr_field::my_attri_field_impl, attribute_macro::my_custom_attribute_impl,
    type_param::process_type_params_impl,
};

#[proc_macro_attribute]
pub fn my_custom_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    my_custom_attribute_impl(args, input)
}

#[proc_macro_attribute]
pub fn my_field_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    my_attri_field_impl(args, input)
}

#[proc_macro_attribute]
pub fn process_type_params(args: TokenStream, input: TokenStream) -> TokenStream {
    process_type_params_impl(args, input)
}
