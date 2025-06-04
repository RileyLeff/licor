pub mod errors;
pub mod macros;
pub mod devices;
pub mod configs;
pub mod parsing;
pub mod parser;

pub use errors::ParseError;
pub use macros::{VariableDef, DataType, parse_licor_toml};
pub use devices::{LiCorDevice, LiCorMetadata, Device6800, Device6400};
pub use configs::{LiCorConfig, ConfigStandard, ConfigFluorometer, ConfigAquatic, ConfigSoil};
pub use parsing::RawLiCorFile;
pub use parser::{
    LiCorParser, LiCorData, VariableInfo,
    LiCor6800Standard, LiCor6800Fluorometer, LiCor6800Aquatic, LiCor6800Soil
};

// Test the macro system
include_variable_definitions!("licor.toml");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_definitions_parsing() {
        let variables = parse_licor_toml().expect("Should parse licor.toml");
        assert!(!variables.is_empty(), "Should have parsed some variables");
        
        // Look for a specific variable we know exists
        let aperture = variables.iter().find(|v| v.internal_name == "Aperture");
        assert!(aperture.is_some(), "Should find Aperture variable");
        
        let aperture = aperture.unwrap();
        assert_eq!(aperture.display_label, "Aperture");
        assert_eq!(aperture.units, Some("cm2"));
    }

    #[test]
    fn test_variable_definitions_static() {
        let variables = &*VARIABLE_DEFINITIONS;
        assert!(!variables.is_empty(), "Static variables should be populated");
        
        // Test that we have a reasonable number of variables
        assert!(variables.len() > 100, "Should have many variables");
    }

    #[test]
    fn test_device_trait() {
        use std::collections::HashMap;
        
        let mut header = HashMap::new();
        header.insert("Console s/n".to_string(), "68C-901292".to_string());
        header.insert("Console ver".to_string(), "Bluestem v.2.1.13".to_string());
        header.insert("Head s/n".to_string(), "68H-581292".to_string());
        
        // Should validate successfully
        assert!(Device6800::validate_header(&header).is_ok());
        
        // Should parse metadata successfully
        let metadata = Device6800::parse_metadata(&header).unwrap();
        assert_eq!(metadata.device_serial, "68C-901292");
        assert_eq!(metadata.console_version, "Bluestem v.2.1.13");
    }

    #[test]
    fn test_config_trait() {
        let standard_vars = ConfigStandard::expected_variables();
        assert!(standard_vars.contains(&"A"));
        assert!(standard_vars.contains(&"E"));
        
        let fluor_vars = ConfigFluorometer::expected_variables();
        assert!(fluor_vars.contains(&"A"));
        assert!(fluor_vars.contains(&"PhiPS2"));
        
        // Test validation with missing variables
        let incomplete_cols = vec!["obs".to_string(), "A".to_string()];
        assert!(ConfigStandard::validate_columns(&incomplete_cols).is_err());
    }
}