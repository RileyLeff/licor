#!/usr/bin/env Rscript

# Debug what's happening with the data frame
setwd("r-client")
devtools::load_all(".", quiet = TRUE)

sample_file <- "inst/extdata/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1"

cat("🔍 Debugging data frame structure...\n")

df <- file_to_dataframe(sample_file, "data.frame", "6800", "fluorometer", TRUE)

cat("Class of result:", class(df), "\n")
cat("Type of result:", typeof(df), "\n")
cat("Structure:\n")
str(df)

cat("\nTrying different approaches:\n")
cat("length(df):", length(df), "\n")

if (is.list(df)) {
  cat("It's a list with", length(df), "elements\n")
  cat("First few element names:", paste(head(names(df), 5), collapse = ", "), "\n")
  
  if (length(df) > 0) {
    cat("First element class:", class(df[[1]]), "\n")
    cat("First element length:", length(df[[1]]), "\n")
  }
}

if (is.data.frame(df)) {
  cat("It's a proper data.frame\n")
  cat("nrow:", nrow(df), "ncol:", ncol(df), "\n")
} else {
  cat("Not a standard data.frame\n")
}