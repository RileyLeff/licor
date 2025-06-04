#!/usr/bin/env Rscript

# Test the actual LI-COR functionality!
setwd("r-client")

cat("🧪 Testing LI-COR client functionality...\n")

# Load the package from local directory
cat("📦 Loading licorclient package from local directory...\n")

# Try loading with devtools
if (requireNamespace("devtools", quietly = TRUE)) {
  devtools::load_all(".", quiet = TRUE)
  cat("✅ Package loaded with devtools!\n")
} else {
  # Try loading the shared library directly
  cat("Loading shared library directly...\n")
  if (file.exists("src/licorclient.so")) {
    dyn.load("src/licorclient.so")
    # Source the wrapper functions
    if (file.exists("R/extendr-wrappers.R")) {
      source("R/extendr-wrappers.R")
      cat("✅ Functions loaded directly!\n")
    } else {
      cat("❌ Wrapper functions not found\n")
    }
  } else {
    cat("❌ Shared library not found\n")
  }
}

# Test file
sample_file <- "inst/extdata/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1"

if (file.exists(sample_file)) {
  cat("✅ Sample file found!\n")
  
  # Test 1: Parquet conversion
  cat("\n📁 Testing convert() to Parquet...\n")
  output_file <- tempfile(fileext = ".parquet")
  
  tryCatch({
    convert(sample_file, output_file, "6800", "fluorometer")
    
    if (file.exists(output_file)) {
      cat("✅ Parquet conversion successful!\n")
      cat("   File size:", file.size(output_file), "bytes\n")
      unlink(output_file)
    } else {
      cat("❌ Parquet file not created\n")
    }
  }, error = function(e) {
    cat("❌ convert() error:", conditionMessage(e), "\n")
  })
  
  # Test 2: DataFrame with preserved names
  cat("\n📊 Testing file_to_dataframe() with preserved names...\n")
  tryCatch({
    df_orig <- file_to_dataframe(sample_file, "data.frame", "6800", "fluorometer", TRUE)
    cat("✅ DataFrame (preserved names) successful!\n")
    cat("   Dimensions:", nrow(df_orig), "rows ×", ncol(df_orig), "columns\n")
    cat("   Sample problematic names:", paste(head(names(df_orig)[grepl("Δ|/|%", names(df_orig))], 3), collapse = ", "), "\n")
  }, error = function(e) {
    cat("❌ DataFrame (preserved) error:", conditionMessage(e), "\n")
  })
  
  # Test 3: DataFrame with cleaned names  
  cat("\n🧹 Testing file_to_dataframe() with cleaned names...\n")
  tryCatch({
    df_clean <- file_to_dataframe(sample_file, "data.frame", "6800", "fluorometer", FALSE)
    cat("✅ DataFrame (cleaned names) successful!\n")
    cat("   Dimensions:", nrow(df_clean), "rows ×", ncol(df_clean), "columns\n")
    cat("   Sample cleaned names:", paste(head(names(df_clean), 5), collapse = ", "), "\n")
  }, error = function(e) {
    cat("❌ DataFrame (cleaned) error:", conditionMessage(e), "\n")
  })
  
} else {
  cat("❌ Sample file not found:", sample_file, "\n")
}

cat("\n🎉 Testing complete!\n")