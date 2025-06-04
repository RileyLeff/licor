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
├── Cargo.toml                 # Workspace definition
├── licor.toml                 # Variable definitions (single source of truth)
├── core/                      # Core Rust library
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs            # Public API, traits, main types
│   │   ├── macros.rs         # Variable definition generation
│   │   ├── devices/          # Device implementations
│   │   │   ├── mod.rs
│   │   │   ├── li6800.rs
│   │   │   └── li6400.rs
│   │   ├── configs/          # Configuration implementations  
│   │   │   ├── mod.rs
│   │   │   ├── standard.rs
│   │   │   └── fluorometer.rs
│   │   ├── parsing.rs        # Raw parsing logic
│   │   └── errors.rs         # Error types
│   └── build.rs              # Build script if needed
├── cli/                       # Command-line interface
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── python/                    # Future: Python bindings (PyO3/maturin)
├── r/                         # Future: R bindings (extendr)
└── test-data/                 # Sample LI-COR files for testing
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

## Future Client Libraries

### Python Client (PyO3)
- Direct file-to-DataFrame conversion: `licor.parse_6800_fluorometer("data.txt")` 
- Access to underlying Rust parsing with error handling
- Integration with Polars and Pandas ecosystems
- Rich metadata preservation in DataFrame attributes

### R Client (extendr)
- R-native interface: `parse_licor_6800(files, config="fluorometer")`
- Direct integration with data.frame and tibble
- Metadata as attributes compatible with R conventions
- Error handling that respects R's error model

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