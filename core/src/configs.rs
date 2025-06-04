use crate::{ParseError, VARIABLE_DEFINITIONS};

/// Trait for measurement configuration validation
pub trait LiCorConfig {
    const CONFIG_NAME: &'static str;
    
    /// Variables expected for this measurement configuration
    fn expected_variables() -> &'static [&'static str];
    
    /// Validate that required variables are present in the columns
    fn validate_columns(columns: &[String]) -> Result<(), ParseError> {
        let expected = Self::expected_variables();
        
        for &required_var in expected {
            if !columns.iter().any(|col| col == required_var) {
                return Err(ParseError::MissingRequiredVariable {
                    variable: required_var.to_string(),
                    config: Self::CONFIG_NAME.to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Check if a variable is known (exists in our definitions)
    fn is_known_variable(variable: &str) -> bool {
        VARIABLE_DEFINITIONS.iter().any(|def| def.internal_name == variable)
    }
}

/// Standard gas exchange measurements
pub struct ConfigStandard;

impl LiCorConfig for ConfigStandard {
    const CONFIG_NAME: &'static str = "standard";
    
    fn expected_variables() -> &'static [&'static str] {
        &[
            "obs",      // observation number
            "A",        // net CO2 assimilation
            "E",        // transpiration rate  
            "Ca",       // reference CO2 concentration
            "Ci",       // intercellular CO2 concentration
            "gsw",      // stomatal conductance to water vapor
            "gbw",      // boundary layer conductance to water vapor
            "Tleaf",    // leaf temperature
            "Tair",     // air temperature
            "Flow",     // flow rate
            "Pa",       // atmospheric pressure
        ]
    }
}

/// Gas exchange with chlorophyll fluorescence
pub struct ConfigFluorometer;

impl LiCorConfig for ConfigFluorometer {
    const CONFIG_NAME: &'static str = "fluorometer";
    
    fn expected_variables() -> &'static [&'static str] {
        &[
            // Standard gas exchange variables
            "obs", "A", "E", "Ca", "Ci", "gsw", "gbw", "Tleaf", "Tair", "Flow", "Pa",
            // Fluorescence variables
            "F",        // fluorescence yield
            "Fm'",      // maximum fluorescence in light
            "Fo'",      // minimum fluorescence in light  
            "PhiPS2",   // quantum yield of PSII
            "ETR",      // electron transport rate
            "qP",       // photochemical quenching
            "NPQ",      // non-photochemical quenching
        ]
    }
}

/// Aquatic chamber measurements (future implementation)
pub struct ConfigAquatic;

impl LiCorConfig for ConfigAquatic {
    const CONFIG_NAME: &'static str = "aquatic";
    
    fn expected_variables() -> &'static [&'static str] {
        &[
            "obs",
            "Qabs",     // flux absorbed by algae
            "Qin",      // flux incident on sample
            "Qout",     // flux leaving sample
            "A",        // net CO2 uptake
            "E",        // water loss (if applicable)
            "Pa",       // atmospheric pressure
        ]
    }
}

/// Soil respiration measurements (future implementation)  
pub struct ConfigSoil;

impl LiCorConfig for ConfigSoil {
    const CONFIG_NAME: &'static str = "soil";
    
    fn expected_variables() -> &'static [&'static str] {
        &[
            "obs",
            "A",        // soil CO2 efflux
            "Tsoil",    // soil temperature
            "VWC",      // volumetric water content
            "Pa",       // atmospheric pressure
            "Flow",     // flow rate
        ]
    }
}