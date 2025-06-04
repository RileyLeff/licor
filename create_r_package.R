#!/usr/bin/env Rscript

# Create R package the proper way using usethis and rextendr

library(usethis)
library(rextendr)

# Override the nesting check - we want this in our monorepo
options(usethis.allow_nested_project = TRUE)

# Create the package structure 
cat("Creating R package with usethis...\n")
usethis::create_package("licorclient", open = FALSE)

# Rename the folder to r-client for our monorepo structure
file.rename("licorclient", "r-client")

# Change to the package directory
setwd("r-client")

# Set up extendr scaffolding
cat("Setting up extendr scaffolding...\n")
rextendr::use_extendr()

cat("Package created successfully!\n")
cat("Generated files:\n")

# List the structure
files <- list.files(".", recursive = TRUE, full.names = TRUE)
for (f in files) {
  cat("  ", f, "\n")
}