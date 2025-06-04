use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid file format for device {device}")]
    InvalidFileFormat { device: String },
    
    #[error("Missing required header field: {field}")]
    MissingRequiredHeader { field: String },
    
    #[error("Unknown variable: {variable}")]
    UnknownVariable { variable: String },
    
    #[error("Missing required variable '{variable}' for config '{config}'")]
    MissingRequiredVariable { variable: String, config: String },
    
    #[error("Malformed data section: expected {expected} columns, found {found}")]
    MalformedDataSection { expected: usize, found: usize },
    
    #[error("Data type error for variable '{variable}': cannot convert '{value}' to {expected_type}")]
    DataTypeError {
        value: String,
        expected_type: String,
        variable: String,
    },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),
    
    #[error("Invalid header format: {message}")]
    InvalidHeaderFormat { message: String },
    
    #[error("Empty or invalid data section")]
    EmptyDataSection,
}