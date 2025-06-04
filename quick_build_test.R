#!/usr/bin/env Rscript

# Quick test to see if the build can complete now
library(rextendr)

setwd("r-client")

cat("Checking build status...\n")

# Try a quick document to see if build completes
tryCatch({
  cat("Attempting rextendr::document()...\n")
  rextendr::document()
  
  cat("✅ Build completed successfully!\n")
  
  # Test if functions are available
  if (exists("convert") && exists("file_to_dataframe")) {
    cat("✅ Functions loaded successfully!\n")
    
    # Quick functionality test
    sample_file <- "inst/extdata/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1"
    if (file.exists(sample_file)) {
      cat("✅ Sample file found, ready for testing!\n")
    } else {
      cat("❌ Sample file not found\n")
    }
  } else {
    cat("❌ Functions not found\n")
  }
  
}, error = function(e) {
  cat("❌ Build error:", conditionMessage(e), "\n")
  
  # Check if it's still compiling
  if (grepl("cargo", conditionMessage(e)) || grepl("build", conditionMessage(e))) {
    cat("🔄 Build might still be in progress...\n")
  }
})