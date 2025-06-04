use pyo3::prelude::*;
use pyo3::exceptions::{PyValueError, PyIOError, PyRuntimeError};
use licor_core::{
    LiCor6800Standard, LiCor6800Fluorometer, LiCor6800Aquatic, LiCor6800Soil,
    ParseError, LiCorData
};
use std::path::Path;
use polars::prelude::*;
use pyo3_polars::PyDataFrame;

/// Convert a LI-COR file to Parquet format
/// 
/// Args:
///     file: Path to the input LI-COR file
///     output: Path for the output Parquet file  
///     device: Device type ("6800" or "6400")
///     config: Measurement configuration ("standard", "fluorometer", "aquatic", "soil")
///
/// Raises:
///     ValueError: Invalid device/config combination or malformed data
///     IOError: File read/write errors
///     RuntimeError: Other parsing errors
#[pyfunction]
fn convert(file: &str, output: &str, device: &str, config: &str) -> PyResult<()> {
    let data = parse_file_internal(file, device, config)?;
    
    // Write to Parquet
    let mut output_file = std::fs::File::create(output)
        .map_err(|e| PyIOError::new_err(format!("Failed to create output file: {}", e)))?;
    
    ParquetWriter::new(&mut output_file)
        .finish(&mut data.dataframe.clone())
        .map_err(|e| PyIOError::new_err(format!("Failed to write Parquet file: {}", e)))?;
    
    Ok(())
}

/// Convert a LI-COR file directly to a DataFrame
///
/// Args:
///     file: Path to the input LI-COR file
///     format: Output format ("polars" or "pandas")
///     device: Device type ("6800" or "6400") 
///     config: Measurement configuration ("standard", "fluorometer", "aquatic", "soil")
///
/// Returns:
///     DataFrame in the requested format
///
/// Raises:
///     ValueError: Invalid device/config combination, unsupported format, or malformed data
///     IOError: File read errors
///     RuntimeError: Missing optional dependencies or other parsing errors
#[pyfunction]
fn file_to_dataframe(file: &str, format: &str, device: &str, config: &str) -> PyResult<PyObject> {
    let data = parse_file_internal(file, device, config)?;
    
    match format {
        "polars" => {
            // Check if polars is available
            Python::with_gil(|py| {
                match py.import("polars") {
                    Ok(_) => {
                        // Convert Polars DataFrame to Python via pyo3-polars
                        let py_df = PyDataFrame(data.dataframe);
                        Ok(py_df.into_pyobject(py)?.into_any().unbind())
                    }
                    Err(_) => Err(PyRuntimeError::new_err(
                        "polars is not installed. Install with: uv add licor-client[polars]"
                    ))
                }
            })
        }
        "pandas" => {
            // Check if pandas is available
            Python::with_gil(|py| {
                let _pandas = py.import("pandas")
                    .map_err(|_| PyRuntimeError::new_err(
                        "pandas is not installed. Install with: uv add licor-client[pandas]"
                    ))?;
                
                // For now, pandas support is not implemented
                Err(PyRuntimeError::new_err(
                    "pandas support is not yet fully implemented. Use format='polars' instead."
                ))
            })
        }
        _ => Err(PyValueError::new_err(format!(
            "Unsupported format '{}'. Supported formats: 'polars', 'pandas'", format
        )))
    }
}

/// Internal function to parse a file with device/config validation
fn parse_file_internal(file: &str, device: &str, config: &str) -> PyResult<LiCorData> {
    // Validate file exists
    if !Path::new(file).exists() {
        return Err(PyIOError::new_err(format!("File not found: {}", file)));
    }
    
    // Parse based on device/config combination
    let data = match (device, config) {
        ("6800", "standard") => {
            let parser = LiCor6800Standard::new();
            parser.parse_file(file)
        }
        ("6800", "fluorometer") => {
            let parser = LiCor6800Fluorometer::new();
            parser.parse_file(file)
        }
        ("6800", "aquatic") => {
            let parser = LiCor6800Aquatic::new();
            parser.parse_file(file)
        }
        ("6800", "soil") => {
            let parser = LiCor6800Soil::new();
            parser.parse_file(file)
        }
        ("6400", _) => {
            return Err(PyValueError::new_err("LI-6400 support not yet implemented"));
        }
        _ => {
            return Err(PyValueError::new_err(format!(
                "Invalid device/config combination: device='{}', config='{}'. \
                 Supported: device='6800'|'6400', config='standard'|'fluorometer'|'aquatic'|'soil'",
                device, config
            )));
        }
    };
    
    // Convert ParseError to appropriate Python exception
    data.map_err(|e| match e {
        ParseError::Io(io_err) => PyIOError::new_err(format!("IO error: {}", io_err)),
        ParseError::InvalidFileFormat { device } => PyValueError::new_err(format!("Invalid file format for device: {}", device)),
        ParseError::MissingRequiredHeader { field } => PyValueError::new_err(format!("Missing required header field: {}", field)),
        ParseError::MissingRequiredVariable { variable, config } => PyValueError::new_err(format!("Missing required variable '{}' for config '{}'", variable, config)),
        ParseError::UnknownVariable { variable } => PyValueError::new_err(format!("Unknown variable: {}", variable)),
        ParseError::MalformedDataSection { expected, found } => PyValueError::new_err(format!("Malformed data section: expected {} columns, found {}", expected, found)),
        ParseError::DataTypeError { value, expected_type, variable } => PyValueError::new_err(format!("Data type error in variable '{}': cannot convert '{}' to {}", variable, value, expected_type)),
        ParseError::InvalidHeaderFormat { message } => PyValueError::new_err(format!("Invalid header format: {}", message)),
        ParseError::EmptyDataSection => PyValueError::new_err("Empty data section"),
        ParseError::TomlParse(e) => PyValueError::new_err(format!("TOML parsing error: {}", e)),
    })
}

/// Python module definition
#[pymodule]
fn licor_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    m.add_function(wrap_pyfunction!(file_to_dataframe, m)?)?;
    Ok(())
}