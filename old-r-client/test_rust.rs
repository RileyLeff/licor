
use extendr_api::prelude::*;

/// A simple test function
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello from Rust!"
}

// Macro to generate exports
extendr_module! {
    mod test;
    fn hello_world;
}

