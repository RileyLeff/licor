# licor_client

R client for converting LI-COR instrument data to analysis-ready formats.

## Overview

`licor_client` provides R bindings for converting LI-COR instrument data (particularly the LI-6800 Portable Photosynthesis System) from its proprietary format to analysis-ready data frames and Parquet files. Built on a robust Rust core with type-safe parsing and comprehensive error handling.

## Features

- **Type-safe parsing** with compile-time validation of device/config combinations
- **Rich metadata preservation** from LI-COR headers (device serial, calibration data, etc.)
- **Flexible output formats**: R data.frame, tibble, or Parquet files
- **Column name handling**: Choose between preserving original scientific names or R-friendly names
- **Robust error handling** with informative error messages
- **High performance** Rust-based conversion engine

## Installation

### System Requirements

- **Rust toolchain**: Install from [rustup.rs](https://rustup.rs/)
- **R packages**: `tibble` (optional, for tibble output format)

### Install from Source

```r
# Install development dependencies
install.packages(c("devtools", "roxygen2", "testthat"))

# Install rextendr for building Rust extensions
install.packages("rextendr")

# Clone and install
devtools::install_github("rileyleff/licor-parser", subdir = "r-client")
```

### Development Build

```bash
cd r-client
R -e "rextendr::document()"
R CMD INSTALL .
```

## Usage

### Basic Conversion

```r
library(licor_client)

# Convert to Parquet format
convert(
  file = "fluorometer_data.txt",
  output = "fluorometer_data.parquet", 
  device = "6800",
  config = "fluorometer"
)

# Convert to R data.frame
df <- file_to_dataframe(
  file = "fluorometer_data.txt",
  format = "data.frame",
  device = "6800", 
  config = "fluorometer"
)
```

### Column Name Handling

LI-COR files contain many variable names with special characters that require backticks in R:

```r
# Preserve original scientific names (default)
df_orig <- file_to_dataframe(
  file = "data.txt", 
  format = "data.frame",
  device = "6800",
  config = "fluorometer",
  preserve_names = TRUE
)

# Access with backticks
head(df_orig$`ΔCO2`)      # Delta CO2
head(df_orig$`Fv/Fm`)     # Photosystem II efficiency  
head(df_orig$`Fan_%`)     # Fan percentage

# Convert to R-friendly names
df_clean <- file_to_dataframe(
  file = "data.txt",
  format = "data.frame", 
  device = "6800",
  config = "fluorometer",
  preserve_names = FALSE
)

# Access with standard R syntax
head(df_clean$delta_co2)  # Converted from ΔCO2
head(df_clean$fv_per_fm)  # Converted from Fv/Fm
head(df_clean$fan_pct)    # Converted from Fan_%
```

### Tibble Output

```r
# Requires tibble package
if (requireNamespace("tibble", quietly = TRUE)) {
  tbl <- file_to_dataframe(
    file = "data.txt",
    format = "tibble",
    device = "6800", 
    config = "fluorometer"
  )
  print(tbl)
}
```

## Supported Configurations

### Devices
- **LI-6800**: Portable Photosynthesis System ✅
- **LI-6400**: Legacy system (planned)

### Measurement Configurations
- **standard**: Basic gas exchange measurements
- **fluorometer**: Gas exchange + chlorophyll fluorescence
- **aquatic**: Aquatic chamber measurements (planned)
- **soil**: Soil respiration measurements (planned)

## Data Types and Conversion

The package automatically handles type conversion based on LI-COR variable definitions:

- **Numeric variables**: Converted to R numeric (double) vectors
- **Integer variables**: Converted to R integer vectors  
- **Boolean variables**: Converted to R logical vectors
- **Text variables**: Kept as character vectors
- **Failed conversions**: Gracefully fall back to character type

## Error Handling

The package provides informative error messages for common issues:

```r
# File not found
convert("missing.txt", "out.parquet", "6800", "fluorometer")
#> Error: File not found: missing.txt

# Invalid device/config combination  
file_to_dataframe("data.txt", "data.frame", "invalid", "fluorometer")
#> Error: Invalid device/config combination: device='invalid', config='fluorometer'

# Missing tibble package
file_to_dataframe("data.txt", "tibble", "6800", "fluorometer") 
#> Error: tibble package required for format="tibble". Install with: install.packages("tibble")
```

## Performance

Built on a high-performance Rust core:
- Processes large datasets efficiently
- Zero-copy data conversion where possible
- Minimal memory overhead
- Type-safe parsing prevents silent data corruption

## Example Data

The package includes sample LI-6800 fluorometer data for testing:

```r
# Get path to sample data
sample_file <- system.file("extdata", 
  "2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1", 
  package = "licor_client")

# Convert sample data
df <- file_to_dataframe(
  file = sample_file,
  format = "data.frame",
  device = "6800",
  config = "fluorometer"
)

dim(df)  # Should show ~10 rows x ~295 columns
```

## Development

### Building from Source

1. Install Rust: https://rustup.rs/
2. Install R development tools: `install.packages(c("devtools", "rextendr"))`
3. Build: `R -e "rextendr::document()"`
4. Test: `R -e "devtools::test()"`
5. Install: `R CMD INSTALL .`

### Testing

```r
devtools::test()
```

### Package Structure

```
r-client/
├── DESCRIPTION              # R package metadata
├── NAMESPACE               # Auto-generated exports
├── R/
│   └── licor_client.R      # R wrapper functions with roxygen docs
├── src/
│   ├── Makevars           # Build configuration
│   ├── entrypoint.c       # C entry point
│   └── rust/
│       ├── Cargo.toml     # Rust dependencies  
│       └── src/
│           └── lib.rs     # extendr bindings to licor-core
├── man/                   # Auto-generated documentation
├── tests/
│   └── testthat/          # R tests
└── inst/
    └── extdata/           # Sample data files
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes and add tests
4. Ensure all tests pass: `devtools::test()`
5. Submit a pull request

## Citation

When using this package in scientific work, please cite:

```
Leff, R. (2025). licor_client: R Client for LI-COR Instrument Data Conversion. 
R package version 0.1.0.
```