if (!requireNamespace("rextendr", quietly = TRUE)) {
  install.packages("rextendr", repos = "https://cran.r-project.org")
}

# Check if rextendr is available
if (requireNamespace("rextendr", quietly = TRUE)) {
  cat("rextendr is available\n")
} else {
  cat("rextendr installation failed\n")
}