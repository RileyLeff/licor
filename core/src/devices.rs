use crate::ParseError;
use std::collections::HashMap;

/// Trait for device-specific parsing and validation
pub trait LiCorDevice {
    const DEVICE_NAME: &'static str;
    
    /// Validate that the header contains required device-specific fields
    fn validate_header(header: &HashMap<String, String>) -> Result<(), ParseError>;
    
    /// Parse device-specific metadata from header
    fn parse_metadata(header: &HashMap<String, String>) -> Result<LiCorMetadata, ParseError>;
}

/// Device metadata extracted from file headers
#[derive(Debug, Clone)]
pub struct LiCorMetadata {
    pub device_serial: String,
    pub console_version: String,
    pub head_serial: Option<String>,
    pub head_version: Option<String>,
    pub chamber_type: Option<String>,
    pub chamber_serial: Option<String>,
    pub fluorometer_serial: Option<String>,
    pub calibration_date: Option<String>,
}

/// LI-6800 Portable Photosynthesis System
pub struct Device6800;

impl LiCorDevice for Device6800 {
    const DEVICE_NAME: &'static str = "LI-6800";
    
    fn validate_header(header: &HashMap<String, String>) -> Result<(), ParseError> {
        // Check for required LI-6800 fields
        let required_fields = ["Console s/n", "Console ver", "Head s/n"];
        
        for field in required_fields {
            if !header.contains_key(field) {
                return Err(ParseError::MissingRequiredHeader { 
                    field: field.to_string() 
                });
            }
        }
        
        // Validate that this is actually a 6800
        if let Some(version) = header.get("Console ver") {
            if !version.contains("Bluestem") {
                return Err(ParseError::InvalidFileFormat { 
                    device: Self::DEVICE_NAME.to_string() 
                });
            }
        }
        
        Ok(())
    }
    
    fn parse_metadata(header: &HashMap<String, String>) -> Result<LiCorMetadata, ParseError> {
        let device_serial = header.get("Console s/n")
            .ok_or_else(|| ParseError::MissingRequiredHeader { 
                field: "Console s/n".to_string() 
            })?
            .clone();
            
        let console_version = header.get("Console ver")
            .ok_or_else(|| ParseError::MissingRequiredHeader { 
                field: "Console ver".to_string() 
            })?
            .clone();
            
        Ok(LiCorMetadata {
            device_serial,
            console_version,
            head_serial: header.get("Head s/n").cloned(),
            head_version: header.get("Head ver").cloned(),
            chamber_type: header.get("Chamber type").cloned(),
            chamber_serial: header.get("Chamber s/n").cloned(),
            fluorometer_serial: header.get("Fluorometer").cloned(),
            calibration_date: header.get("Factory cal date").cloned(),
        })
    }
}

/// LI-6400 Portable Photosynthesis System (future implementation)
pub struct Device6400;

impl LiCorDevice for Device6400 {
    const DEVICE_NAME: &'static str = "LI-6400";
    
    fn validate_header(_header: &HashMap<String, String>) -> Result<(), ParseError> {
        // TODO: Implement LI-6400 validation when we have sample files
        Err(ParseError::InvalidFileFormat { 
            device: "LI-6400 support not yet implemented".to_string() 
        })
    }
    
    fn parse_metadata(_header: &HashMap<String, String>) -> Result<LiCorMetadata, ParseError> {
        // TODO: Implement LI-6400 metadata parsing
        Err(ParseError::InvalidFileFormat { 
            device: "LI-6400 support not yet implemented".to_string() 
        })
    }
}