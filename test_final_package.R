#!/usr/bin/env Rscript

# Test the full LI-COR client implementation
library(rextendr)

setwd("r-client")

cat("Testing complete LI-COR R client...\n")

# Document and build the package
tryCatch({
  cat("Building package with rextendr::document()...\n")
  rextendr::document()
  
  cat("✅ Package documented successfully!\n")
  
  # Test file conversion
  sample_file <- "inst/extdata/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1"
  
  if (file.exists(sample_file)) {
    cat("✅ Sample file found\n")
    
    # Test Parquet conversion
    output_file <- tempfile(fileext = ".parquet")
    
    cat("Testing convert() function...\n")
    tryCatch({
      convert(sample_file, output_file, "6800", "fluorometer")
      
      if (file.exists(output_file)) {
        cat("✅ Parquet conversion successful!\n")
        cat("   File size:", file.size(output_file), "bytes\n")
      } else {
        cat("❌ Parquet file not created\n")
      }
      
      unlink(output_file)
      
    }, error = function(e) {
      cat("❌ convert() error:", conditionMessage(e), "\n")
    })
    
    # Test DataFrame conversion
    cat("Testing file_to_dataframe() function...\n")
    tryCatch({
      # Test with preserved names
      df_orig <- file_to_dataframe(sample_file, "data.frame", "6800", "fluorometer", TRUE)
      cat("✅ DataFrame conversion (preserved names) successful!\n")
      cat("   Dimensions:", nrow(df_orig), "rows x", ncol(df_orig), "columns\n")
      cat("   First few column names:", paste(head(names(df_orig), 5), collapse = ", "), "\n")
      
      # Test with cleaned names
      df_clean <- file_to_dataframe(sample_file, "data.frame", "6800", "fluorometer", FALSE)
      cat("✅ DataFrame conversion (cleaned names) successful!\n")
      cat("   First few cleaned names:", paste(head(names(df_clean), 5), collapse = ", "), "\n")
      
    }, error = function(e) {
      cat("❌ file_to_dataframe() error:", conditionMessage(e), "\n")
      print(e)
    })
    
  } else {
    cat("❌ Sample file not found:", sample_file, "\n")
  }
  
}, error = function(e) {
  cat("❌ Build error:", conditionMessage(e), "\n")
  print(e)
})

cat("Test complete!\n")