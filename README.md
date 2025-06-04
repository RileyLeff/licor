# LI-COR Data Parser

This project provides a robust and type-safe system for converting proprietary LI-COR instrument data (primarily from the LI-6800 Portable Photosynthesis System) into analysis-ready formats like Parquet, with client libraries for Python and R.

## Project Motivation

LI-COR instruments output data in a complex, undocumented format that can be challenging to use directly for scientific analysis. This format often mixes metadata and data, uses variable names that may not align with their descriptions, and lacks a formal schema. This project aims to alleviate these challenges by providing tools to parse these files reliably and convert them into standardized, well-typed formats.

The goal is to create a robust, type-safe conversion system that transforms LI-COR's proprietary format into analysis-ready Parquet files with rich metadata preservation.

## Features

*   **Core Rust Library (`licor-core`)**: Provides the central parsing logic, ensuring type safety and robust error handling.
*   **Command-Line Interface (`licor`)**: A CLI tool for batch converting LI-COR files to Parquet.
*   **Python Client (`licor-client`)**: A Python library (using PyO3/Maturin) for seamless integration into Python data analysis workflows, offering conversion to Parquet or Polars/Pandas DataFrames.
*   **R Client (`licorclient`)**: An R library (using extendr) for using the parsing capabilities directly within R, offering conversion to Parquet or R data.frame/tibble objects.
*   **Type-Safe Parsing**: Utilizes Rust's type system to handle different device and measurement configurations, minimizing errors.
*   **Variable Definition System**: Leverages a comprehensive `licor.toml` file to define over 440 variables, including their units, descriptions, and data types.
*   **Supported Devices**:
    *   LI-6800 (fully implemented)
*   **Supported Configurations**:
    *   `standard`: Basic gas exchange measurements.
    *   `fluorometer`: Gas exchange with chlorophyll fluorescence.
*   **Output Formats**:
    *   Parquet (with metadata preservation)
    *   Python: Polars DataFrame, Pandas DataFrame (planned)
    *   R: `data.frame`, `tibble`
*   **Error Handling**: Provides user-friendly error messages for common parsing issues.

## Repository Structure

The repository is organized as a Cargo workspace:

```
licor-parser/
├── Cargo.toml                 # Workspace definition
├── licor.toml                 # Central variable definitions
├── core/                      # Core Rust parsing library
├── cli/                       # Command-line interface tool
├── python-client/             # Python client library
├── r-client/                  # R client library
├── example_data/              # Sample LI-COR data files for testing
└── plan.md                    # Project planning document
```

## Installation

### Core Library & CLI (from source)

To build the core library and CLI tool from source, you'll need the Rust toolchain installed.

```bash
git clone https://github.com/your-username/licor-parser.git # Replace with your repo URL
cd licor-parser
cargo build --release
# The CLI tool will be available at target/release/licor
```

### Python Client

The Python client can be installed using pip (ideally with `uv` or in a virtual environment).

```bash
# Basic installation (from PyPI - once published)
# pip install licor-client
# uv add licor-client

# With Polars support
# pip install licor-client[polars]
# uv add licor-client[polars]

# With Pandas support (once fully implemented)
# pip install licor-client[pandas]
# uv add licor-client[pandas]

# To install from source (after cloning the repository):
cd python-client
maturin develop # Or pip install .
```
Requires Python 3.8+.

### R Client

The R client can be installed like a standard R package. You'll need Rust and the `rextendr` setup if installing from source.

```R
# Once available on CRAN (example)
# install.packages("licorclient")

# To install from source (after cloning the repository):
# Ensure you have the Rust toolchain and devtools installed in R
# install.packages("devtools")
# devtools::install("r-client") # Assuming your working directory is the repo root
```

## Usage

### Command-Line Interface (`licor`)

The CLI tool converts LI-COR files to Parquet format.

```bash
licor convert \
  --device 6800 \
  --config fluorometer \
  --input "example_data/*.txt" \
  --output "converted_data/" \
  --verbose
```

**CLI Options:**

*   `--device <DEVICE>`: Device type (e.g., `6800`).
*   `--config <CONFIG>`: Measurement configuration (e.g., `standard`, `fluorometer`).
*   `--input <INPUT>`: Input file(s) (supports glob patterns).
*   `--output <OUTPUT>`: Output directory for Parquet files.
*   `-v, --verbose`: Enable verbose output.
*   `-h, --help`: Print help information.

### Python Client (`licor-client`)

```python
import licor_client

# Convert a LI-COR file to Parquet
licor_client.convert(
    file="example_data/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1",
    output="output_data.parquet",
    device="6800",
    config="fluorometer"
)

# Convert a LI-COR file directly to a Polars DataFrame
df_polars = licor_client.file_to_dataframe(
    file="example_data/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1",
    format="polars",
    device="6800",
    config="fluorometer"
)
print(f"Processed {df_polars.height} observations with {df_polars.width} variables.")

# Convert to a Pandas DataFrame (ensure pandas extra is installed)
# df_pandas = licor_client.file_to_dataframe(
#     file="example_data/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1",
#     format="pandas", # Support for pandas is planned
#     device="6800",
#     config="fluorometer"
# )
```

### R Client (`licorclient`)

```R
library(licorclient)

# Convert a LI-COR file to Parquet
convert(
  file = "example_data/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1",
  output = "output_data.parquet",
  device = "6800",
  config = "fluorometer"
)

# Convert a LI-COR file directly to an R data.frame
df_r <- file_to_dataframe(
  file = "example_data/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1",
  format = "data.frame", # or "tibble"
  device = "6800",
  config = "fluorometer",
  preserve_names = TRUE # Set to FALSE to get R-friendly names
)
print(paste("Processed", nrow(df_r), "observations with", ncol(df_r), "variables."))
```

## Supported Devices and Configurations

*   **Devices**:
    *   `"6800"`: LI-6800 Portable Photosynthesis System

*   **Configurations**:
    *   `"standard"`: Basic gas exchange measurements
    *   `"fluorometer"`: Gas exchange + chlorophyll fluorescence
    *   `"aquatic"`: Aquatic chamber measurements (in future if someone asks)
    *   `"soil"`: Soil respiration measurements (in future if someone asks)

## License

This project is licensed under the MIT License OR Apache License 2.0. See `Cargo.toml` files for details.

## Contributing

Contributions are welcome! Please refer to the `plan.md` for project status and future directions. If you encounter any issues or have suggestions, please open an issue on the GitHub repository.

## Author

*   Riley Leff
