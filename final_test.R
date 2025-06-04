#!/usr/bin/env Rscript

# Very quick test to check completion
setwd("r-client")

cat("Quick completion check...\n")

# Look for the shared library
if (file.exists("src/licorclient.so") || file.exists("src/licorclient.dylib")) {
  cat("✅ Shared library found! Build is complete!\n")
} else {
  cat("🔄 Still building...\n")
}

# Check current cargo status
result <- system("cd src/rust && cargo check --quiet", intern = TRUE)
if (attr(result, "status") == 0 || is.null(attr(result, "status"))) {
  cat("✅ Cargo check passes - Rust compilation successful!\n")
} else {
  cat("❌ Cargo check failed\n")
}