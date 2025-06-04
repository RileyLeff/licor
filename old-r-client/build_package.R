library(rextendr)

# Document the package (compile Rust and generate R wrappers)
cat("Documenting package...\n")
result <- try(rextendr::document(), silent = FALSE)

if (inherits(result, "try-error")) {
  cat("Error during documentation:\n")
  print(result)
} else {
  cat("Package documented successfully!\n")
  
  # Check if the shared library was created
  if (file.exists("src/licor_client.so") || file.exists("src/licor_client.dll")) {
    cat("Shared library created successfully!\n")
  } else {
    cat("No shared library found\n")
  }
  
  # List generated files
  cat("Generated files:\n")
  if (dir.exists("R")) {
    r_files <- list.files("R", full.names = TRUE)
    for (f in r_files) {
      cat("  ", f, "\n")
    }
  }
}