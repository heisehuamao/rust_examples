#![allow(warnings)]
use std::fmt;

use my_macro::MyTrait;
use my_macro::my_custom_attribute;
use my_macro::my_field_attribute;
use my_macro::process_type_params;

// This will map perfectly to your `MyMacroArgs` struct
#[my_custom_attribute(name = "worker_pool", timeout = 500, verbose)]
fn execute_task() {
    println!("Doing heavy lifting...");
}

#[my_custom_attribute(name = "worker_pool", timeout = 500, verbose)]
struct AttrStr {
    a: i32,
    b: String,
}

#[my_field_attribute]
struct DatabaseUser {
    id: u64,

    #[field_info(skip)] // Parsed into `skip: true`
    password_hash: String,

    #[field_info_2(another_param = "fdfsd")]
    alias_name: String,
}

#[my_field_attribute]
enum UserRole {
    Admin,

    #[variant_info(rename = "guest_user")] // Parsed into `rename: Some(...)`
    Guest,

    #[variant_info_2(custom_id = 123, rename = "custom X")]
    Custom,
}

// 1. Apply your attribute macro to a struct with mixed generics
#[process_type_params]
struct MyData<
    'a,
    const N: usize,
    #[my_type_attr(a = false, b, c)]
    #[doc] T: Clone + std::fmt::Debug = String,
> {
    reference: &'a str,
    payload: T,
    array: [i32; N],
}

#[derive(MyTrait, Default)]
#[my_trait(msg = "Running custom macro code!", count = 3)]
struct UserConfig {
    key: String,

    #[my_trait(rename = "fdsfds")]
    value: String,
}

#[derive(MyTrait)]
#[my_trait(msg = "This one only prints once")]
enum AppConfig {
    Path,
    #[my_trait(skip, version = -1)]
    Os,
}

fn main() {
    execute_task();
    // The macro leaves the original struct completely intact,
    // so it compiles and runs normally here.
    let data = MyData {
        reference: "hello",
        payload: 42,
        array: [1, 2, 3],
    };

    println!("Payload: {:?}", data.payload);

    let user = UserConfig::default();
    user.print_message(); // Prints the message 3 times

    let app = AppConfig::Path;
    app.print_message(); // Prints the message 1 time (default)
}
