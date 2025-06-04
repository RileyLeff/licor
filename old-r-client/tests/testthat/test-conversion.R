test_that("convert function works with sample data", {
  skip_on_cran()
  
  sample_file <- system.file("extdata", 
    "2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1", 
    package = "licorclient")
  
  if (!file.exists(sample_file)) {
    skip("Sample data file not found")
  }
  
  output_file <- tempfile(fileext = ".parquet")
  
  expect_no_error(
    convert(
      file = sample_file,
      output = output_file,
      device = "6800",
      config = "fluorometer"
    )
  )
  
  expect_true(file.exists(output_file))
  expect_gt(file.size(output_file), 0)
  
  # Clean up
  unlink(output_file)
})

test_that("file_to_dataframe works with sample data", {
  skip_on_cran()
  
  sample_file <- system.file("extdata", 
    "2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1", 
    package = "licorclient")
  
  if (!file.exists(sample_file)) {
    skip("Sample data file not found")
  }
  
  # Test data.frame output with preserved names
  df <- file_to_dataframe(
    file = sample_file,
    format = "data.frame",
    device = "6800",
    config = "fluorometer",
    preserve_names = TRUE
  )
  
  expect_s3_class(df, "data.frame")
  expect_gt(nrow(df), 0)
  expect_gt(ncol(df), 0)
  
  # Should have some expected columns
  expect_true("obs" %in% names(df))
  expect_true("A" %in% names(df))
  
  # Test with cleaned names
  df_clean <- file_to_dataframe(
    file = sample_file,
    format = "data.frame", 
    device = "6800",
    config = "fluorometer",
    preserve_names = FALSE
  )
  
  expect_s3_class(df_clean, "data.frame")
  expect_equal(nrow(df), nrow(df_clean))
  expect_equal(ncol(df), ncol(df_clean))
  
  # Names should be different when preserve_names = FALSE
  expect_false(identical(names(df), names(df_clean)))
})

test_that("tibble format works when tibble is available", {
  skip_on_cran()
  skip_if_not_installed("tibble")
  
  sample_file <- system.file("extdata", 
    "2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1", 
    package = "licorclient")
  
  if (!file.exists(sample_file)) {
    skip("Sample data file not found")
  }
  
  tbl <- file_to_dataframe(
    file = sample_file,
    format = "tibble",
    device = "6800", 
    config = "fluorometer"
  )
  
  expect_s3_class(tbl, "tbl_df")
  expect_s3_class(tbl, "data.frame")
})

test_that("error handling works correctly", {
  # Test file not found
  expect_error(
    convert(
      file = "nonexistent_file.txt",
      output = "output.parquet", 
      device = "6800",
      config = "fluorometer"
    ),
    "File not found"
  )
  
  # Test invalid device/config combination
  sample_file <- system.file("extdata", 
    "2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1", 
    package = "licorclient")
  
  if (file.exists(sample_file)) {
    expect_error(
      file_to_dataframe(
        file = sample_file,
        format = "data.frame",
        device = "invalid",
        config = "fluorometer"
      ),
      "Invalid device/config combination"
    )
    
    expect_error(
      file_to_dataframe(
        file = sample_file,
        format = "invalid_format",
        device = "6800",
        config = "fluorometer"
      ),
      "Unsupported format"
    )
  }
})

test_that("tibble error when package not available", {
  skip_on_cran()
  
  sample_file <- system.file("extdata", 
    "2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1", 
    package = "licorclient")
  
  if (!file.exists(sample_file)) {
    skip("Sample data file not found") 
  }
  
  # Mock tibble not being available by temporarily removing it from search path
  if ("package:tibble" %in% search()) {
    detach("package:tibble", unload = TRUE)
    on.exit(library(tibble), add = TRUE)
  }
  
  expect_error(
    file_to_dataframe(
      file = sample_file,
      format = "tibble",
      device = "6800",
      config = "fluorometer"
    ),
    "tibble package required"
  )
})