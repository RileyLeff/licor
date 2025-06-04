use crate::ParseError;
use std::collections::HashMap;

/// Raw LI-COR file structure extracted from string content
#[derive(Debug, Clone)]
pub struct RawLiCorFile {
    pub header: HashMap<String, String>,
    pub column_categories: Vec<String>,
    pub column_names: Vec<String>,
    pub units: Vec<String>,
    pub data_rows: Vec<Vec<String>>,
}

impl RawLiCorFile {
    /// Parse a LI-COR file from string content
    pub fn parse(content: &str) -> Result<Self, ParseError> {
        let mut lines = content.lines().map(|s| s.trim()).collect::<Vec<_>>();
        
        // Find [Header] section
        let header_start = lines.iter().position(|line| *line == "[Header]")
            .ok_or_else(|| ParseError::InvalidHeaderFormat { 
                message: "Missing [Header] section".to_string() 
            })?;
            
        // Find [Data] section
        let data_start = lines.iter().position(|line| *line == "[Data]")
            .ok_or_else(|| ParseError::InvalidHeaderFormat { 
                message: "Missing [Data] section".to_string() 
            })?;
            
        if data_start <= header_start {
            return Err(ParseError::InvalidHeaderFormat { 
                message: "[Data] section must come after [Header] section".to_string() 
            });
        }
        
        // Parse header section
        let header = Self::parse_header(&lines[header_start + 1..data_start])?;
        
        // Parse data section
        let data_lines = &lines[data_start + 1..];
        if data_lines.len() < 3 {
            return Err(ParseError::EmptyDataSection);
        }
        
        let column_categories = Self::parse_tab_separated_line(data_lines[0])?;
        let column_names = Self::parse_tab_separated_line(data_lines[1])?;
        let units = Self::parse_tab_separated_line(data_lines[2])?;
        
        // Handle column count mismatches by padding shorter vectors
        let max_cols = column_categories.len().max(column_names.len()).max(units.len());
        
        let mut column_categories = column_categories;
        let mut column_names = column_names;
        let mut units = units;
        
        // Pad vectors to the same length
        column_categories.resize(max_cols, String::new());
        column_names.resize(max_cols, String::new());
        units.resize(max_cols, String::new());
        
        let num_cols = max_cols;
        
        // Parse data rows (skip first 3 lines which are headers)
        let mut data_rows = Vec::new();
        for (_line_num, line) in data_lines.iter().skip(3).enumerate() {
            if line.trim().is_empty() {
                continue; // Skip empty lines
            }
            
            let row = Self::parse_tab_separated_line(line)?;
            if row.len() != num_cols {
                // For now, pad short rows with empty strings or truncate long rows
                // This is more lenient than failing immediately
                let mut adjusted_row = row;
                adjusted_row.resize(num_cols, String::new());
                data_rows.push(adjusted_row);
                continue;
            }
            data_rows.push(row);
        }
        
        if data_rows.is_empty() {
            return Err(ParseError::EmptyDataSection);
        }
        
        Ok(RawLiCorFile {
            header,
            column_categories,
            column_names,
            units,
            data_rows,
        })
    }
    
    fn parse_header(lines: &[&str]) -> Result<HashMap<String, String>, ParseError> {
        let mut header = HashMap::new();
        
        for line in lines {
            if line.is_empty() {
                continue;
            }
            
            // Handle various header formats
            if let Some((key, value)) = Self::parse_header_line(line) {
                header.insert(key, value);
            }
        }
        
        Ok(header)
    }
    
    fn parse_header_line(line: &str) -> Option<(String, String)> {
        // Handle different separator patterns in header
        if let Some(tab_pos) = line.find('\t') {
            // Tab-separated key-value
            let key = line[..tab_pos].trim().to_string();
            let value = line[tab_pos + 1..].trim().to_string();
            return Some((key, value));
        }
        
        // Handle colon-separated values (like "SysConst:AvgTime	4")
        if line.contains(':') && line.contains('\t') {
            if let Some(tab_pos) = line.find('\t') {
                let key = line[..tab_pos].trim().to_string();
                let value = line[tab_pos + 1..].trim().to_string();
                return Some((key, value));
            }
        }
        
        // Handle complex stability definition lines
        if line.contains("Stability Definition:") {
            if let Some(tab_pos) = line.find('\t') {
                let key = line[..tab_pos].trim().to_string();
                let value = line[tab_pos + 1..].trim().to_string();
                return Some((key, value));
            }
        }
        
        None
    }
    
    fn parse_tab_separated_line(line: &str) -> Result<Vec<String>, ParseError> {
        // Split by tabs and handle empty values
        // Note: Some lines may have trailing tabs that create empty fields
        let mut values: Vec<String> = line.split('\t')
            .map(|s| s.trim().to_string())
            .collect();
            
        if values.is_empty() {
            return Err(ParseError::InvalidHeaderFormat { 
                message: "Empty line in data section".to_string() 
            });
        }
        
        // Remove trailing empty values that come from trailing tabs
        while values.last() == Some(&String::new()) {
            values.pop();
        }
        
        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_parsing_sample_file() {
        let content = std::fs::read_to_string("../example_data/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1")
            .expect("Should be able to read sample file");
            
        let raw_file = RawLiCorFile::parse(&content).expect("Should parse sample file");
        
        // Test header parsing
        assert!(raw_file.header.contains_key("Console s/n"));
        assert_eq!(raw_file.header.get("Console s/n").unwrap(), "68C-901292");
        
        // Test data structure
        assert!(!raw_file.column_names.is_empty());
        assert!(!raw_file.data_rows.is_empty());
        assert_eq!(raw_file.column_names.len(), raw_file.units.len());
        assert_eq!(raw_file.column_names.len(), raw_file.column_categories.len());
        
        // Check that we have the expected variables
        assert!(raw_file.column_names.contains(&"obs".to_string()));
        assert!(raw_file.column_names.contains(&"A".to_string()));
        assert!(raw_file.column_names.contains(&"E".to_string()));
    }
}