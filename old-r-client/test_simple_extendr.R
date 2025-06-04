# Create a minimal test to see if extendr basics work
library(rextendr)

# Create a simple Rust function
rust_code <- '
use extendr_api::prelude::*;

/// A simple test function
/// @export
#[extendr]
fn hello_world() -> &\'static str {
    "Hello from Rust!"
}

// Macro to generate exports
extendr_module! {
    mod test;
    fn hello_world;
}
'

# Try to create and compile this simple function
tryCatch({
  writeLines(rust_code, "test_rust.rs")
  
  # Test if we can at least compile something simple
  cat("Testing basic extendr functionality...\n")
  
}, error = function(e) {
  cat("Error:", conditionMessage(e), "\n")
})