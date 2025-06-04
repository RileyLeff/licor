# Simple test to see if we can load the library
cat("Testing basic R functionality...\n")

# Test file existence
sample_file <- "inst/extdata/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1"
if (file.exists(sample_file)) {
  cat("Sample file exists:", sample_file, "\n")
  
  # Test basic file reading
  lines <- readLines(sample_file, n = 10)
  cat("First few lines:\n")
  for (i in 1:min(3, length(lines))) {
    cat(i, ":", substr(lines[i], 1, 80), "...\n")
  }
} else {
  cat("Sample file not found:", sample_file, "\n")
  cat("Files in inst/extdata:\n")
  if (dir.exists("inst/extdata")) {
    files <- list.files("inst/extdata")
    for (f in files) {
      cat("  ", f, "\n")
    }
  }
}

cat("Basic R test complete.\n")