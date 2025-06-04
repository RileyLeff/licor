# LI-COR Data Parser

## Project Motivation

LI-COR instruments (particularly the LI-6800 Portable Photosynthesis System) output data in a proprietary, undocumented format that is extremely difficult to work with for scientific analysis. The format combines:

- Tab-separated metadata in a non-standard header format
- Mixed data types in columns without proper type declarations  
- Variable names that don't match their descriptions
- Complex nested groupings that violate standard data conventions
- No formal schema or documentation from LI-COR

This creates significant friction for plant ecophysiologists who need to analyze photosynthesis and fluorescence data. Researchers currently spend substantial time manually cleaning and reformatting data instead of focusing on science.

**Goal**: Create a robust, type-safe conversion system that transforms LI-COR's proprietary format into analysis-ready Parquet files with rich metadata preservation.

## ✅ Implementation Status (COMPLETED)

**As of 2025-06-04**: Core Rust library, CLI tool, and Python client library are fully implemented and working with real LI-6800 data files.

### Completed Components:

1. **✅ Cargo Workspace** - Clean separation between core library and CLI binary
2. **✅ Variable Definition System** - Macro-generated parsing of 440+ variables from `licor.toml`
3. **✅ Type-Safe Device + Config Traits** - Compile-time validation prevents wrong parser usage
4. **✅ Three-Stage Processing Pipeline** - Raw parsing → validation → type conversion with fallbacks
5. **✅ Robust Error Handling** - User-friendly errors using `thiserror` 
6. **✅ CLI Tool** - Batch processing with glob patterns, progress reporting, and verbose output
7. **✅ Parquet Output** - Analysis-ready format with proper data types and metadata preservation
8. **✅ Test Coverage** - End-to-end tests with real LI-6800 fluorometer data files
9. **✅ Python Client Library** - PyO3/maturin-based bindings ready for PyPI publication

### Working CLI Interface:
```bash
# Convert files with explicit device/config specification
cargo run --bin licor -- convert \
  --device 6800 \
  --config fluorometer \
  --input "example_data/*" \
  --output "converted/" \
  --verbose

# Successfully tested with 2 real LI-6800 files (10 rows × 295 columns each)
```

### Type Safety Achievements:
- **Compile-time validation** of device/config combinations via Rust traits
- **Impossible to use wrong parser** for a given dataset  
- **Graceful type conversion** with fallback to string when numeric parsing fails
- **Rich metadata preservation** from LI-COR headers (device serial, version, calibration data)
- **Duplicate column handling** with automatic renaming for DataFrame compatibility

## Implementation Strategy

### Core Architecture: Trait-Based Device + Configuration System

We use Rust's type system to ensure compile-time safety for different device and measurement configuration combinations:

```rust
// Device trait - handles device-specific parsing/validation
trait LiCorDevice {
    const DEVICE_NAME: &'static str;
    fn validate_header(header: &HashMap<String, String>) -> Result<(), ParseError>;
    fn parse_header(header: &HashMap<String, String>) -> Result<LiCorMetadata, ParseError>;
}

// Config trait - handles measurement setup expectations
trait LiCorConfig {
    const CONFIG_NAME: &'static str;
    fn expected_variables() -> &'static [&'static str];
    fn validate_columns(columns: &[String]) -> Result<(), ParseError>;
}

// Generic parser parameterized by device and config
struct LiCorParser<D: LiCorDevice, C: LiCorConfig> {
    _device: PhantomData<D>,
    _config: PhantomData<C>,
}
```

### Three-Stage Processing Pipeline

1. **Raw Parsing**: Simple string-based extraction of header and data sections
2. **Validation**: Device-specific header validation + config-specific column validation  
3. **Type Conversion**: Transform to strongly-typed DataFrame using variable definitions

### Macro-Generated Variable Definitions

A compile-time macro reads `licor.toml` (comprehensive variable mapping) to generate:

- Static variable definitions with units, descriptions, and data types
- Variable sets for different measurement configurations (standard, fluorometer, etc.)
- Lookup tables for validation and type conversion

```rust
include_variable_definitions!("licor.toml");
// Generates VARIABLE_DEFINITIONS and variable_sets module
```

## Library Structure

### Core Types

```rust
// Device implementations
struct Device6800;
struct Device6400; // Future

// Configuration implementations  
struct ConfigStandard;    // Basic gas exchange
struct ConfigFluorometer; // Gas exchange + chlorophyll fluorescence
struct ConfigAquatic;     // Future - aquatic chamber
struct ConfigSoil;        // Future - soil respiration

// Type-safe parser combinations
type LiCor6800Standard = LiCorParser<Device6800, ConfigStandard>;
type LiCor6800Fluorometer = LiCorParser<Device6800, ConfigFluorometer>;
```

### Error Handling

Strict validation with user-friendly error messages:

```rust
enum ParseError {
    InvalidFileFormat { device: String },
    MissingRequiredHeader { field: String }, 
    UnknownVariable { variable: String },
    MissingRequiredVariable { variable: String, config: String },
    MalformedDataSection { expected: usize, found: usize },
    DataTypeError { value: String, expected_type: String, variable: String },
    // ... others
}
```

**Philosophy**: Fail fast with clear error messages rather than silently accepting malformed data. Research data integrity is paramount.

### Output Format

- **Primary**: Parquet files with embedded metadata (device info, variable descriptions, units)
- **Metadata preservation**: All header information, calibration data, and variable metadata stored in Parquet metadata
- **Type safety**: Proper data types (f64, i64, String, bool) based on variable definitions

## CLI Tool

Simple, explicit interface requiring user to specify device and configuration:

```bash
licor convert \
  --device 6800 \
  --config fluorometer \
  --input data/*.txt \
  --output converted/
```

### CLI Features

- **Required parameters**: Device type and configuration must be explicit
- **Batch processing**: Handle multiple files with progress indication
- **Validation**: Clear error reporting for malformed or unexpected data
- **Single output format**: Parquet only (other formats out of scope)

### CLI Help Output

```
Convert LI-COR instrument data to analysis-ready Parquet format

Usage: licor convert --device <DEVICE> --config <CONFIG> --input <INPUT> --output <OUTPUT>

Options:
      --device <DEVICE>    Device type [possible values: 6800, 6400]
      --config <CONFIG>    Measurement configuration [possible values: standard, fluorometer, aquatic, soil]
      --input <INPUT>      Input files (supports glob patterns)
      --output <OUTPUT>    Output directory for Parquet files
  -v, --verbose           Enable verbose output
  -h, --help              Print help
```

## Workspace Structure

```
licor-parser/
├── Cargo.toml                 # Workspace definition (includes python-client)
├── licor.toml                 # Variable definitions (single source of truth)
├── core/                      # Core Rust library
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs            # Public API, traits, main types
│   │   ├── macros.rs         # Variable definition generation
│   │   ├── devices.rs        # Device implementations (6800, 6400)
│   │   ├── configs.rs        # Configuration implementations  
│   │   ├── parsing.rs        # Raw parsing logic
│   │   ├── parser.rs         # Type-safe parser implementation
│   │   └── errors.rs         # Error types
├── cli/                       # Command-line interface
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── python-client/             # ✅ Python bindings (PyO3/maturin) 
│   ├── Cargo.toml            # PyO3 dependencies
│   ├── pyproject.toml        # Python packaging
│   ├── README.md
│   ├── src/
│   │   └── lib.rs            # PyO3 bindings to licor-core
│   ├── python/
│   │   └── licor_client/
│   │       ├── __init__.py   # Python API
│   │       ├── __init__.pyi  # Type stubs
│   │       └── py.typed      # PEP 561 marker
│   └── tests/
│       └── test_integration.py
├── example_data/              # Sample LI-COR files for testing
│   ├── 2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1
│   └── 2025-05-30-1203_logdata_F2
└── output/                    # Generated Parquet files
```

## Implementation Details

### Variable Definition Macro

The macro processes `licor.toml` to generate:

```rust
// Generated at compile time
pub const VARIABLE_DEFINITIONS: &[(&str, VariableDef)] = &[
    ("obs", VariableDef { 
        internal_name: "obs",
        display_label: "Number observations logged",
        units: None,
        description: "Number observations logged",
        data_type: DataType::Integer,
    }),
    // ... 280+ variables from TOML
];

pub mod variable_sets {
    pub const GAS_EXCHANGE: &[&str] = &["A", "E", "Ca", "Ci", "gsw", "gbw"];
    pub const FLUORESCENCE: &[&str] = &["F", "Fm", "Fo", "PhiPS2", "ETR"];
    pub const STANDARD: &[&str] = &[GAS_EXCHANGE, SYSTEM_OBS].concat();
    pub const FLUOROMETER: &[&str] = &[GAS_EXCHANGE, FLUORESCENCE, SYSTEM_OBS].concat();
}
```

### Two-Stage Parsing

```rust
// Stage 1: Raw string extraction
struct RawLiCorFile {
    header: HashMap<String, String>,
    column_names: Vec<String>,
    units: Vec<String>,
    data_rows: Vec<Vec<String>>,
}

// Stage 2: Validation + type conversion
impl<D: LiCorDevice, C: LiCorConfig> LiCorParser<D, C> {
    fn parse_file(&self, path: &str) -> Result<LiCorData, ParseError> {
        let raw = RawLiCorFile::parse(&std::fs::read_to_string(path)?)?;
        D::validate_header(&raw.header)?;
        C::validate_columns(&raw.column_names)?;
        self.build_typed_data(raw)
    }
}
```

### Extensibility

- **New devices**: Implement `LiCorDevice` trait
- **New configurations**: Implement `LiCorConfig` trait + add variable set to TOML
- **New variables**: Add to `licor.toml`, regenerate at compile time
- **Future formats**: Trait-based design allows different parsing strategies

## ✅ Python Client Library (COMPLETED)

**As of 2025-06-04**: Fully functional Python client library ready for PyPI publication.

### Implementation Overview

The Python client provides a clean, type-safe interface to the Rust core library via PyO3 bindings built with maturin. The implementation prioritizes scientific reproducibility by requiring all parameters to be explicit (no defaults) and provides excellent error handling by mapping Rust `ParseError` types to appropriate Python exceptions.

### API Design

**Final API:**
```python
import licor_client

# File conversion to Parquet
licor_client.convert(
    file="data.txt", 
    output="data.parquet", 
    device="6800", 
    config="fluorometer"
)

# Direct DataFrame conversion
df = licor_client.file_to_dataframe(
    file="data.txt", 
    format="polars",  # "polars" currently supported, "pandas" planned
    device="6800", 
    config="fluorometer"
)
```

### Technical Implementation

**Architecture:**
- **PyO3 0.24** - Latest stable PyO3 with modern API
- **pyo3-polars 0.21** - Native polars DataFrame conversion without serialization overhead
- **Maturin build system** - Industry standard for Rust-Python packages
- **Workspace integration** - Python client as workspace member sharing core dependencies

**Key Design Decisions:**

1. **Zero-Copy DataFrame Conversion**: Uses `pyo3-polars` for direct memory mapping between Rust polars DataFrames and Python polars objects, avoiding expensive serialization.

2. **Optional Dependencies Pattern**: Implements proper Python extras syntax:
   ```bash
   uv add licor-client[polars]     # Polars support
   uv add licor-client[pandas]     # Pandas support (planned)  
   uv add licor-client[dataframes] # Both
   ```

3. **Runtime Dependency Checking**: Graceful error messages if optional dependencies not installed rather than import-time failures.

4. **Error Mapping Strategy**: Each Rust `ParseError` variant maps to appropriate Python exception:
   - `ParseError::Io` → `IOError`
   - `ParseError::InvalidFileFormat` → `ValueError`
   - `ParseError::MissingRequiredHeader` → `ValueError`
   - etc.

**Project Structure:**
```
python-client/
├── Cargo.toml              # PyO3 bindings, workspace member
├── pyproject.toml          # Python packaging with optional deps
├── README.md               # Python-specific documentation
├── src/
│   └── lib.rs              # PyO3 bindings to licor-core
├── python/
│   └── licor_client/
│       ├── __init__.py     # Python API
│       ├── __init__.pyi    # Type stubs
│       └── py.typed        # PEP 561 marker
└── tests/
    ├── __init__.py
    └── test_integration.py # Integration tests with real data
```

### Implementation Challenges Overcome

1. **Version Compatibility**: Required upgrading from PyO3 0.22 to 0.24 and polars 0.46 to 0.48 to resolve native library linking conflicts.

2. **DataFrame Conversion**: PyO3's API evolution required using `into_pyobject()?.into_any().unbind()` pattern for proper PyObject conversion.

3. **Build Environment**: Maturin requires pip in virtual environment, resolved by using `uv pip install pip` while maintaining uv-centric workflow.

4. **API Consistency**: Maintained exact same explicit parameter requirements as CLI tool to ensure scientific reproducibility.

### Testing & Validation

**Integration Testing:** Comprehensive test suite using real LI-6800 data files:
- Successfully processes 10 rows × 295 columns of fluorometer data
- Validates both `convert()` and `file_to_dataframe()` functions
- Tests error handling for invalid parameters and missing files
- Verifies optional dependency detection and error messages

**Performance:** Zero-copy conversion achieves excellent performance:
- 115KB Parquet output from sample data
- Instant DataFrame creation with 295 columns
- No serialization overhead between Rust and Python

### Installation & Usage

**Installation:**
```bash
# Basic installation
uv add licor-client

# With polars support  
uv add licor-client[polars]

# Development build
cd python-client
maturin develop
```

**Usage Example:**
```python
import licor_client

# Convert LI-6800 fluorometer data
df = licor_client.file_to_dataframe(
    file="data/fluorometer_measurement.txt",
    format="polars", 
    device="6800",
    config="fluorometer"
)

print(f"Processed {df.height} observations with {df.width} variables")
# Output: Processed 10 observations with 295 variables
```

### PyPI Readiness

The library is production-ready for initial PyPI publication:
- ✅ Clean build process with `maturin build --release`
- ✅ Comprehensive type hints and documentation
- ✅ Integration tests with real scientific data
- ✅ Proper error handling and user-friendly messages
- ✅ Optional dependencies correctly configured
- ✅ PEP 561 compliance for type checking

**Publication Commands:**
```bash
cd python-client
maturin build --release          # Build wheels
maturin publish                  # Publish to PyPI
```

## Available Data for Implementation

The implementation will have access to:

1. **`licor.toml`**: Comprehensive variable definitions with 280+ variables organized by category, including units, descriptions, and data types
2. **Sample data files**: 
   - `2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1`: LI-6800 with fluorometer attachment
   - `2025-05-30-1203_logdata_F2`: Another fluorometer dataset
3. **Variable categories**: System observations, gas exchange, fluorescence, leak correction, computed results, etc.

## Success Criteria

1. **Type safety**: Impossible to use wrong device/config combinations at compile time
2. **Data integrity**: Unknown variables cause errors rather than silent data corruption  
3. **Rich metadata**: All variable descriptions, units, and calibration data preserved
4. **Performance**: Fast conversion of large datasets
5. **Usability**: Simple CLI that researchers can use without deep technical knowledge
6. **Extensibility**: Easy to add support for new LI-COR instruments and measurement types

The goal is to eliminate the pain point of LI-COR data processing for the plant ecophysiology research community while maintaining the highest standards for data integrity and type safety.

## 🚧 Next Phase: R Client Library

**Planned Implementation**: R bindings using the `extendr` framework to provide native R integration.