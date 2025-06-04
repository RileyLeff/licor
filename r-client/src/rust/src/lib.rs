use extendr_api::prelude::*;
use licor_core::{
    LiCor6800Standard, LiCor6800Fluorometer, LiCor6800Aquatic, LiCor6800Soil,
    ParseError, LiCorData
};
use polars::prelude::*;
use std::path::Path;

/// Convert a LI-COR file to Parquet format
/// 
/// @param file Path to the input LI-COR file
/// @param output Path for the output Parquet file  
/// @param device Device type ("6800" or "6400")
/// @param config Measurement configuration ("standard", "fluorometer", "aquatic", "soil")
/// @export
#[extendr]
fn convert(file: &str, output: &str, device: &str, config: &str) -> Result<()> {
    let data = parse_file_internal(file, device, config)?;
    
    // Write to Parquet
    let mut output_file = std::fs::File::create(output)
        .map_err(|e| Error::Other(format!("Failed to create output file: {}", e)))?;
    
    ParquetWriter::new(&mut output_file)
        .finish(&mut data.dataframe.clone())
        .map_err(|e| Error::Other(format!("Failed to write Parquet file: {}", e)))?;
    
    Ok(())
}

/// Convert a LI-COR file directly to a data.frame or tibble
///
/// @param file Path to the input LI-COR file
/// @param format Output format ("data.frame" or "tibble")
/// @param device Device type ("6800" or "6400") 
/// @param config Measurement configuration ("standard", "fluorometer", "aquatic", "soil")
/// @param preserve_names Whether to preserve original LI-COR variable names (TRUE) or convert to R-friendly names (FALSE)
/// @return data.frame or tibble with the converted data
/// @export
#[extendr]
fn file_to_dataframe(
    file: &str, 
    format: &str, 
    device: &str, 
    config: &str, 
    preserve_names: bool
) -> Result<Robj> {
    let data = parse_file_internal(file, device, config)?;
    
    // Convert polars DataFrame to R data.frame
    let r_df = polars_to_r_dataframe(data.dataframe, preserve_names)?;
    
    match format {
        "data.frame" => {
            Ok(r_df)
        }
        "tibble" => {
            // Try to convert to tibble if available
            R!("
                if (!requireNamespace('tibble', quietly = TRUE)) {
                    stop('tibble package required for format=\"tibble\". Install with: install.packages(\"tibble\")')
                }
                tibble::as_tibble({{r_df}})
            ").map_err(|e| Error::Other(format!("Failed to create tibble: {}", e)))
        }
        _ => Err(Error::Other(format!(
            "Unsupported format '{}'. Supported formats: 'data.frame', 'tibble'", format
        )))
    }
}

/// Internal function to parse a file with device/config validation
fn parse_file_internal(file: &str, device: &str, config: &str) -> Result<LiCorData> {
    // Validate file exists
    if !Path::new(file).exists() {
        return Err(Error::Other(format!("File not found: {}", file)));
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
            return Err(Error::Other("LI-6400 support not yet implemented".to_string()));
        }
        _ => {
            return Err(Error::Other(format!(
                "Invalid device/config combination: device='{}', config='{}'. \
                 Supported: device='6800'|'6400', config='standard'|'fluorometer'|'aquatic'|'soil'",
                device, config
            )));
        }
    };
    
    // Convert ParseError to R Error
    data.map_err(|e| match e {
        ParseError::Io(io_err) => Error::Other(format!("IO error: {}", io_err)),
        ParseError::InvalidFileFormat { device } => Error::Other(format!("Invalid file format for device: {}", device)),
        ParseError::MissingRequiredHeader { field } => Error::Other(format!("Missing required header field: {}", field)),
        ParseError::MissingRequiredVariable { variable, config } => Error::Other(format!("Missing required variable '{}' for config '{}'", variable, config)),
        ParseError::UnknownVariable { variable } => Error::Other(format!("Unknown variable: {}", variable)),
        ParseError::MalformedDataSection { expected, found } => Error::Other(format!("Malformed data section: expected {} columns, found {}", expected, found)),
        ParseError::DataTypeError { value, expected_type, variable } => Error::Other(format!("Data type error in variable '{}': cannot convert '{}' to {}", variable, value, expected_type)),
        ParseError::InvalidHeaderFormat { message } => Error::Other(format!("Invalid header format: {}", message)),
        ParseError::EmptyDataSection => Error::Other("Empty data section".to_string()),
        ParseError::TomlParse(e) => Error::Other(format!("TOML parsing error: {}", e)),
    })
}

/// Convert polars DataFrame to R data.frame with optional name cleaning
fn polars_to_r_dataframe(df: DataFrame, preserve_names: bool) -> Result<Robj> {
    // For now, let's simplify and just convert to a basic structure
    // This will need refinement but should compile
    let mut list_data = Vec::new();
    let mut names_vec = Vec::new();
    
    for column in df.get_columns() {
        let name = if preserve_names {
            column.name().to_string()
        } else {
            clean_name_for_r(column.name())
        };
        names_vec.push(name);
        
        // Convert all columns to string for now to ensure it works
        let string_values: Vec<String> = (0..column.len())
            .map(|i| {
                column.get(i).map(|av| format!("{}", av)).unwrap_or_else(|_| "NA".to_string())
            })
            .collect();
        
        list_data.push(string_values);
    }
    
    // Create a simple list structure for R using from_names_and_values
    let r_values: Vec<Robj> = list_data.into_iter()
        .map(|col| col.into())
        .collect();
    
    let r_list = List::from_names_and_values(names_vec, r_values)?;
    
    Ok(r_list.into())
}

/// Clean LI-COR variable names to be R-friendly
fn clean_name_for_r(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            // Replace problematic characters
            'Δ' => "delta_".to_string(),
            '/' => "_per_".to_string(),
            '%' => "_pct".to_string(),
            '@' => "_at_".to_string(),
            ':' => "_".to_string(),
            '\'' => "_prime".to_string(),
            '.' => "_".to_string(),
            '-' => "_".to_string(),
            ' ' => "_".to_string(),
            '⁻' => "_neg".to_string(),
            '²' => "2".to_string(),
            '¹' => "1".to_string(),
            // Keep alphanumeric and underscore
            c if c.is_alphanumeric() || c == '_' => c.to_string(),
            // Replace everything else with underscore
            _ => "_".to_string(),
        })
        .collect::<String>()
        // Clean up multiple consecutive underscores
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
        // Ensure it starts with a letter or dot (R naming convention)
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 && c.is_ascii_digit() {
                format!("X{}", c)
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
        // Convert to lowercase for consistency
        .to_lowercase()
}

// Macro to generate exports
extendr_module! {
    mod licorclient;
    fn convert;
    fn file_to_dataframe;
}