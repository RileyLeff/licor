#!/usr/bin/env Rscript

# Test the basic rextendr-generated package
library(rextendr)

setwd("r-client")

cat("Testing basic rextendr package...\n")

# Try to document and build the hello_world function
tryCatch({
  cat("Running rextendr::document()...\n")
  rextendr::document()
  
  cat("✅ Package documented successfully!\n")
  
  # Test the function
  if (exists("hello_world")) {
    result <- hello_world()
    cat("✅ hello_world() returned:", result, "\n")
  } else {
    cat("❌ hello_world function not found\n")
  }
  
}, error = function(e) {
  cat("❌ Error:", conditionMessage(e), "\n")
  print(e)
})