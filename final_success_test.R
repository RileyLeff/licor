#!/usr/bin/env Rscript

# Final test showing the R client working perfectly!
setwd("r-client")
devtools::load_all(".", quiet = TRUE)

sample_file <- "inst/extdata/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1"

cat("🎉 FINAL SUCCESS TEST!\n")

# Wrapper function to extract the data properly
convert_licor <- function(file, output, device, config) {
  result <- convert(file, output, device, config)
  if (!is.null(result$err)) {
    stop("Conversion failed: ", result$err)
  }
  return("Success")
}

licor_to_dataframe <- function(file, format = "data.frame", device, config, preserve_names = TRUE) {
  result <- file_to_dataframe(file, format, device, config, preserve_names)
  if (!is.null(result$err)) {
    stop("Conversion failed: ", result$err)
  }
  # Convert the list to a proper data.frame
  df <- as.data.frame(result$ok, stringsAsFactors = FALSE)
  return(df)
}

# Test Parquet conversion
cat("\n📁 Testing Parquet conversion...\n")
output_file <- tempfile(fileext = ".parquet")
convert_licor(sample_file, output_file, "6800", "fluorometer")
cat("✅ Parquet file created:", file.size(output_file), "bytes\n")

# Test DataFrame with preserved names
cat("\n📊 Testing DataFrame with preserved names...\n")
df_orig <- licor_to_dataframe(sample_file, "data.frame", "6800", "fluorometer", TRUE)
cat("✅ Data loaded successfully!\n")
cat("   Dimensions:", nrow(df_orig), "rows ×", ncol(df_orig), "columns\n")

# Show some problematic column names
problematic <- names(df_orig)[grepl("Δ|/|%|@|'", names(df_orig))]
cat("   Problematic names found:", length(problematic), "\n")
cat("   Examples:", paste(head(problematic, 5), collapse = ", "), "\n")

# Test DataFrame with cleaned names
cat("\n🧹 Testing DataFrame with cleaned names...\n") 
df_clean <- licor_to_dataframe(sample_file, "data.frame", "6800", "fluorometer", FALSE)
cat("✅ Cleaned data loaded successfully!\n")
cat("   Dimensions:", nrow(df_clean), "rows ×", ncol(df_clean), "columns\n")
cat("   Sample cleaned names:", paste(head(names(df_clean), 5), collapse = ", "), "\n")

# Show transformation examples
if (length(problematic) > 0) {
  cat("\n🔄 Name transformation examples:\n")
  for (i in 1:min(3, length(problematic))) {
    orig_name <- problematic[i]
    orig_pos <- which(names(df_orig) == orig_name)[1]
    if (!is.na(orig_pos) && orig_pos <= ncol(df_clean)) {
      clean_name <- names(df_clean)[orig_pos]
      cat("   ", orig_name, "→", clean_name, "\n")
    }
  }
}

cat("\n🎉 R CLIENT FULLY FUNCTIONAL! 🎉\n")
cat("Ready for scientific analysis of LI-COR data!\n")

# Clean up
unlink(output_file)