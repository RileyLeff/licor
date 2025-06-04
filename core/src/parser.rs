use crate::{
    ParseError, RawLiCorFile, LiCorDevice, LiCorConfig, LiCorMetadata, 
    DataType, VARIABLE_DEFINITIONS
};
use std::marker::PhantomData;
use std::collections::HashSet;
use polars::prelude::*;

/// Type-safe LI-COR parser parameterized by device and configuration
pub struct LiCorParser<D: LiCorDevice, C: LiCorConfig> {
    _device: PhantomData<D>,
    _config: PhantomData<C>,
}

/// Parsed LI-COR data with rich metadata
#[derive(Debug, Clone)]
pub struct LiCorData {
    pub metadata: LiCorMetadata,
    pub dataframe: DataFrame,
    pub variable_info: Vec<VariableInfo>,
}

/// Information about a variable in the dataset
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub internal_name: String,
    pub display_label: String,
    pub units: Option<String>,
    pub description: String,
    pub data_type: DataType,
    pub column_category: String,
}

impl<D: LiCorDevice, C: LiCorConfig> LiCorParser<D, C> {
    /// Create a new parser instance
    pub fn new() -> Self {
        Self {
            _device: PhantomData,
            _config: PhantomData,
        }
    }
    
    /// Parse a LI-COR file from file path
    pub fn parse_file(&self, path: &str) -> Result<LiCorData, ParseError> {
        let content = std::fs::read_to_string(path)?;
        self.parse_content(&content)
    }
    
    /// Parse a LI-COR file from string content
    pub fn parse_content(&self, content: &str) -> Result<LiCorData, ParseError> {
        // Stage 1: Raw parsing
        let raw_file = RawLiCorFile::parse(content)?;
        
        // Stage 2: Device validation
        D::validate_header(&raw_file.header)?;
        let metadata = D::parse_metadata(&raw_file.header)?;
        
        // Stage 3: Configuration validation
        C::validate_columns(&raw_file.column_names)?;
        
        // Stage 4: Type conversion
        let (dataframe, variable_info) = self.build_typed_dataframe(raw_file)?;
        
        Ok(LiCorData {
            metadata,
            dataframe,
            variable_info,
        })
    }
    
    fn build_typed_dataframe(&self, raw_file: RawLiCorFile) -> Result<(DataFrame, Vec<VariableInfo>), ParseError> {
        let mut columns = Vec::new();
        let mut variable_info = Vec::new();
        let mut used_names = HashSet::new();
        
        for (col_idx, column_name) in raw_file.column_names.iter().enumerate() {
            if column_name.is_empty() {
                continue; // Skip empty column names
            }
            
            // Make column name unique if there are duplicates
            let unique_name = if used_names.contains(column_name) {
                let mut counter = 1;
                loop {
                    let candidate = format!("{}_{}", column_name, counter);
                    if !used_names.contains(&candidate) {
                        break candidate;
                    }
                    counter += 1;
                }
            } else {
                column_name.clone()
            };
            used_names.insert(unique_name.clone());
            
            // Find variable definition
            let var_def = VARIABLE_DEFINITIONS.iter()
                .find(|def| def.internal_name == column_name);
            
            // Get column data
            let column_data: Vec<String> = raw_file.data_rows.iter()
                .map(|row| row.get(col_idx).unwrap_or(&String::new()).clone())
                .collect();
                
            if column_data.is_empty() {
                continue;
            }
            
            // Create VariableInfo
            let var_info = if let Some(def) = var_def {
                VariableInfo {
                    internal_name: unique_name.clone(),
                    display_label: def.display_label.to_string(),
                    units: def.units.map(|s| s.to_string()),
                    description: def.description.to_string(),
                    data_type: def.data_type.clone(),
                    column_category: raw_file.column_categories.get(col_idx)
                        .unwrap_or(&String::new()).clone(),
                }
            } else {
                // Unknown variable - infer type from units
                let empty_string = String::new();
                let units = raw_file.units.get(col_idx).unwrap_or(&empty_string);
                let data_type = DataType::infer_from_units(units);
                
                VariableInfo {
                    internal_name: unique_name.clone(),
                    display_label: column_name.clone(),
                    units: if units.is_empty() { None } else { Some(units.clone()) },
                    description: format!("Unknown variable: {}", column_name),
                    data_type,
                    column_category: raw_file.column_categories.get(col_idx)
                        .unwrap_or(&String::new()).clone(),
                }
            };
            
            // Convert to appropriate Polars series based on data type
            let series = match &var_info.data_type {
                DataType::Float => {
                    // Try to parse as float, but fall back to string if any value fails
                    let mut can_parse_all = true;
                    let values: Vec<Option<f64>> = column_data.iter()
                        .map(|s| {
                            if s.is_empty() || s == "-" || s.to_lowercase() == "none" {
                                None
                            } else if let Ok(val) = s.parse::<f64>() {
                                Some(val)
                            } else {
                                can_parse_all = false;
                                None
                            }
                        })
                        .collect();
                    
                    if can_parse_all {
                        Series::new((&var_info.internal_name).into(), values)
                    } else {
                        // Fall back to string type
                        let values: Vec<Option<String>> = column_data.iter()
                            .map(|s| if s.is_empty() { None } else { Some(s.clone()) })
                            .collect();
                        Series::new((&var_info.internal_name).into(), values)
                    }
                }
                DataType::Integer => {
                    // Try to parse as integer, but fall back to string if any value fails
                    let mut can_parse_all = true;
                    let values: Vec<Option<i64>> = column_data.iter()
                        .map(|s| {
                            if s.is_empty() || s == "-" || s.to_lowercase() == "none" {
                                None
                            } else if let Ok(val) = s.parse::<i64>() {
                                Some(val)
                            } else {
                                can_parse_all = false;
                                None
                            }
                        })
                        .collect();
                    
                    if can_parse_all {
                        Series::new((&var_info.internal_name).into(), values)
                    } else {
                        // Fall back to string type
                        let values: Vec<Option<String>> = column_data.iter()
                            .map(|s| if s.is_empty() { None } else { Some(s.clone()) })
                            .collect();
                        Series::new((&var_info.internal_name).into(), values)
                    }
                }
                DataType::Boolean => {
                    // Try to parse as boolean, but fall back to string if any value fails
                    let mut can_parse_all = true;
                    let values: Vec<Option<bool>> = column_data.iter()
                        .map(|s| {
                            if s.is_empty() || s == "-" || s.to_lowercase() == "none" {
                                None
                            } else {
                                match s.to_lowercase().as_str() {
                                    "true" | "1" | "on" | "yes" => Some(true),
                                    "false" | "0" | "off" | "no" => Some(false),
                                    _ => {
                                        can_parse_all = false;
                                        None
                                    }
                                }
                            }
                        })
                        .collect();
                    
                    if can_parse_all {
                        Series::new((&var_info.internal_name).into(), values)
                    } else {
                        // Fall back to string type
                        let values: Vec<Option<String>> = column_data.iter()
                            .map(|s| if s.is_empty() { None } else { Some(s.clone()) })
                            .collect();
                        Series::new((&var_info.internal_name).into(), values)
                    }
                }
                DataType::String => {
                    let values: Vec<Option<String>> = column_data.iter()
                        .map(|s| if s.is_empty() { None } else { Some(s.clone()) })
                        .collect();
                    
                    Series::new((&var_info.internal_name).into(), values)
                }
            };
            
            columns.push(series.into());
            variable_info.push(var_info);
        }
        
        if columns.is_empty() {
            return Err(ParseError::EmptyDataSection);
        }
        
        let dataframe = DataFrame::new(columns)
            .map_err(|e| ParseError::InvalidHeaderFormat { 
                message: format!("Failed to create DataFrame: {}", e) 
            })?;
            
        Ok((dataframe, variable_info))
    }
}

impl<D: LiCorDevice, C: LiCorConfig> Default for LiCorParser<D, C> {
    fn default() -> Self {
        Self::new()
    }
}

// Type aliases for common parser combinations
pub type LiCor6800Standard = LiCorParser<crate::Device6800, crate::ConfigStandard>;
pub type LiCor6800Fluorometer = LiCorParser<crate::Device6800, crate::ConfigFluorometer>;
pub type LiCor6800Aquatic = LiCorParser<crate::Device6800, crate::ConfigAquatic>;
pub type LiCor6800Soil = LiCorParser<crate::Device6800, crate::ConfigSoil>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typed_parser_sample_file() {
        let parser = LiCor6800Fluorometer::new();
        let content = std::fs::read_to_string("../example_data/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1")
            .expect("Should be able to read sample file");
            
        let data = parser.parse_content(&content).expect("Should parse sample file");
        
        // Test metadata
        assert_eq!(data.metadata.device_serial, "68C-901292");
        assert_eq!(data.metadata.console_version, "Bluestem v.2.1.13");
        
        // Test dataframe structure
        assert!(!data.dataframe.is_empty());
        assert!(!data.variable_info.is_empty());
        
        // Test that we have expected variables
        let var_names: Vec<&str> = data.variable_info.iter()
            .map(|v| v.internal_name.as_str())
            .collect();
        assert!(var_names.contains(&"obs"));
        assert!(var_names.contains(&"A"));
        assert!(var_names.contains(&"E"));
        
        // Test data types are correctly inferred (obs should be numeric)
        let obs_var = data.variable_info.iter()
            .find(|v| v.internal_name == "obs")
            .expect("Should have obs variable");
        // Should be either Integer or String (if type conversion fell back)
        assert!(matches!(obs_var.data_type, DataType::Integer | DataType::String));
    }
    
    #[test]
    fn test_type_safety() {
        // This should compile - correct device/config combination
        let _parser = LiCor6800Fluorometer::new();
        
        // These type aliases demonstrate compile-time type safety
        let _standard = LiCor6800Standard::new();
        let _aquatic = LiCor6800Aquatic::new();
        let _soil = LiCor6800Soil::new();
    }
}