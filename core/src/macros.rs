use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VariableDef {
    pub internal_name: &'static str,
    pub display_label: &'static str,
    pub units: Option<&'static str>,
    pub description: &'static str,
    pub data_type: DataType,
}

#[derive(Debug, Clone)]
pub enum DataType {
    Float,
    Integer,
    String,
    Boolean,
}

impl DataType {
    pub fn infer_from_units(units: &str) -> Self {
        match units {
            "" => DataType::String, // Default for empty units
            units if units.contains("V") || 
                     units.contains("µmol") || 
                     units.contains("mmol") ||
                     units.contains("kPa") ||
                     units.contains("C") ||
                     units.contains("m-2") ||
                     units.contains("s-1") ||
                     units.contains("cm2") => DataType::Float,
            _ => DataType::String,
        }
    }
}

#[derive(Deserialize)]
struct TomlConfig {
    #[serde(flatten)]
    sections: HashMap<String, TomlSection>,
}

#[derive(Deserialize)]
struct TomlSection {
    #[serde(flatten)]
    subsections: HashMap<String, TomlSubsection>,
}

#[derive(Deserialize)]
struct TomlSubsection {
    source_table: Option<String>,
    section_title: Option<String>,
    description: Option<String>,
    variables: Vec<TomlVariable>,
}

#[derive(Deserialize)]
struct TomlVariable {
    display_label: String,
    units: String,
    description: String,
    internal_name: String,
}

pub fn parse_licor_toml() -> Result<Vec<VariableDef>, crate::ParseError> {
    let toml_content = include_str!("../../licor.toml");
    let config: TomlConfig = toml::from_str(toml_content)?;
    
    let mut variables = Vec::new();
    
    for (_section_name, section) in config.sections {
        for (_subsection_name, subsection) in section.subsections {
            for var in subsection.variables {
                let data_type = DataType::infer_from_units(&var.units);
                
                let variable_def = VariableDef {
                    internal_name: Box::leak(var.internal_name.into_boxed_str()),
                    display_label: Box::leak(var.display_label.into_boxed_str()),
                    units: if var.units.is_empty() { 
                        None 
                    } else { 
                        Some(Box::leak(var.units.into_boxed_str())) 
                    },
                    description: Box::leak(var.description.into_boxed_str()),
                    data_type,
                };
                
                variables.push(variable_def);
            }
        }
    }
    
    Ok(variables)
}

// Generate variable definitions at compile time
#[macro_export]
macro_rules! include_variable_definitions {
    ($path:expr) => {
        pub static VARIABLE_DEFINITIONS: once_cell::sync::Lazy<Vec<$crate::VariableDef>> = 
            once_cell::sync::Lazy::new(|| {
                $crate::parse_licor_toml().expect("Failed to parse licor.toml")
            });
    };
}