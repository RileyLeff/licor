#!/usr/bin/env Rscript
# Test the CLI functionality from R as a wrapper approach

cat("Testing LI-COR conversion via CLI...\n")

# Test file paths
sample_file <- "inst/extdata/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1"
output_file <- tempfile(fileext = ".parquet")

if (file.exists(sample_file)) {
  cat("✅ Sample file found:", sample_file, "\n")
  
  # Build and run the CLI tool
  cat("Building CLI tool...\n")
  build_result <- system("cd .. && cargo build --bin licor --release", intern = TRUE)
  
  if (attr(build_result, "status") == 0 || is.null(attr(build_result, "status"))) {
    cat("✅ CLI tool built successfully\n")
    
    # Test CLI conversion
    cli_path <- "../target/release/licor"
    cmd <- sprintf('"%s" convert --device 6800 --config fluorometer --input "%s" --output "%s"', 
                   cli_path, sample_file, output_file)
    
    cat("Running command:", cmd, "\n")
    
    cli_result <- system(cmd, intern = TRUE)
    
    if (file.exists(output_file)) {
      cat("✅ Parquet file created successfully\n")
      cat("   File size:", file.size(output_file), "bytes\n")
      
      # Try to read with arrow if available
      if (requireNamespace("arrow", quietly = TRUE)) {
        tryCatch({
          df <- arrow::read_parquet(output_file)
          cat("✅ Parquet file readable with arrow\n")
          cat("   Dimensions:", nrow(df), "rows x", ncol(df), "columns\n")
          cat("   Column names (first 5):", paste(head(names(df), 5), collapse = ", "), "\n")
        }, error = function(e) {
          cat("❌ Error reading Parquet with arrow:", conditionMessage(e), "\n")
        })
      } else {
        cat("ℹ️  arrow package not available for Parquet reading test\n")
      }
      
      # Clean up
      unlink(output_file)
      
    } else {
      cat("❌ Parquet file was not created\n")
      cat("CLI output:\n")
      print(cli_result)
    }
    
  } else {
    cat("❌ CLI build failed\n")
    print(build_result)
  }
  
} else {
  cat("❌ Sample file not found:", sample_file, "\n")
}

cat("CLI test complete.\n")