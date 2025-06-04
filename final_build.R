#!/usr/bin/env Rscript

# Final build attempt with patience
library(rextendr)
setwd("r-client")

cat("Final build attempt...\n")
cat("This should be quick since Rust compilation is done!\n")

start_time <- Sys.time()
rextendr::document()
end_time <- Sys.time()

cat("Build completed in", as.numeric(end_time - start_time), "seconds\n")

# Test the functions
if (exists("convert") && exists("file_to_dataframe")) {
  cat("🎉 SUCCESS! Functions are available!\n")
  cat("Available functions:\n")
  cat("  - convert()\n")
  cat("  - file_to_dataframe()\n")
} else {
  cat("Functions not yet available\n")
}