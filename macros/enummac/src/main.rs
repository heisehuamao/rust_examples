#![allow(warnings)]
use my_macro::DebugMacro;
use std::fmt;

// #[derive(DebugMacro, Clone, Debug)]
// enum LogLevel {
//     Trace,
//     Debug,
//     Info,
//     Warn,
//     Error,
// }

#[derive(DebugMacro)]
#[derive(Clone, Debug)]
#[my_custom_attr(key = "value")]
pub enum LogLevel<'a, T: Clone + fmt::Debug + 'static> {
    #[variant_config(enable = true)]
    Trace,
    Debug,
    Info,
    Warn(String, i32),
    // 4. Changes `fields` from Fields::Unit to Fields::Named
    Error {
        #[need_trim(side = "both", max_len = 100, preserve_newlines = true)]
        message: String,
        code: u32,
        extra: &'a T,
    },
}

#[derive(DebugMacro, Clone, Debug)]
struct MyData<T>
where
    T: Clone + fmt::Debug + 'static, // Your where clause criteria goes here
{
    name: String,
    value: String,
    extra_field: T, // Utilizing the generic parameter constrained by the where clause
}

// #[derive(DebugMacro, Clone, Debug)]
// struct MyData {
//     name: String,
//     value: String,
// }

#[derive(DebugMacro)]
union MyUnion {
    integer: i32,
    floating: f32,
}

impl fmt::Debug for MyUnion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("MyData")
                .field("integer", &self.integer)
                // Alternatively, you could print it as floating:
                // .field("floating", &self.floating)
                .finish()
        }
    }
}

fn main() {
    let lvl = LogLevel::<i32>::Trace;
    let data = MyData {
        name: String::from("nn"),
        value: String::from("fff"),
        extra_field: 11,
    };

    let u = MyUnion { integer: 1 };

    debug_MyData();
    debug_LogLevel();
    debug_MyUnion();
    println!("level: {:?}", lvl);
    println!("data: {:?}", data);
    println!("union: {:?}", u)
}
