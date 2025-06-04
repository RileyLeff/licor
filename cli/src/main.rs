use clap::Parser;
use licor_core::{LiCor6800Standard, LiCor6800Fluorometer, LiCor6800Aquatic, LiCor6800Soil};
use std::path::Path;
use glob::glob;

#[derive(Parser)]
#[command(name = "licor")]
#[command(about = "Convert LI-COR instrument data to analysis-ready Parquet format")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    Convert {
        /// Device type
        #[arg(long, value_enum)]
        device: Device,
        
        /// Measurement configuration
        #[arg(long, value_enum)]
        config: Config,
        
        /// Input files (supports glob patterns)
        #[arg(long)]
        input: String,
        
        /// Output directory for Parquet files
        #[arg(long)]
        output: String,
        
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Device {
    #[value(name = "6800")]
    Li6800,
    #[value(name = "6400")]
    Li6400,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Config {
    Standard,
    Fluorometer,
    Aquatic,
    Soil,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Convert { device, config, input, output, verbose } => {
            convert_files(device, config, input, output, verbose)?;
            Ok(())
        }
    }
}

fn convert_files(
    device: Device, 
    config: Config, 
    input_pattern: String, 
    output_dir: String, 
    verbose: bool
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure output directory exists
    std::fs::create_dir_all(&output_dir)?;
    
    // Find input files using glob pattern
    let input_files: Vec<_> = glob(&input_pattern)?
        .collect::<Result<Vec<_>, _>>()?;
    
    if input_files.is_empty() {
        eprintln!("Error: No files found matching pattern: {}", input_pattern);
        std::process::exit(1);
    }
    
    if verbose {
        println!("Found {} files to convert", input_files.len());
        println!("Device: {:?}", device);
        println!("Config: {:?}", config);
        println!("Output directory: {}", output_dir);
        println!();
    }
    
    let mut successfully_converted = 0;
    let mut failed_conversions = Vec::new();
    
    for input_file in input_files {
        let input_path = input_file.to_string_lossy();
        
        if verbose {
            println!("Converting: {}", input_path);
        }
        
        match convert_single_file(&device, &config, &input_path, &output_dir, verbose) {
            Ok(output_path) => {
                successfully_converted += 1;
                if verbose {
                    println!("  → {}", output_path);
                }
            }
            Err(e) => {
                failed_conversions.push((input_path.to_string(), e.to_string()));
                eprintln!("Error converting {}: {}", input_path, e);
            }
        }
    }
    
    println!();
    println!("Conversion complete:");
    println!("  Successfully converted: {}", successfully_converted);
    println!("  Failed: {}", failed_conversions.len());
    
    if !failed_conversions.is_empty() {
        eprintln!("\nFailed conversions:");
        for (file, error) in failed_conversions {
            eprintln!("  {}: {}", file, error);
        }
        std::process::exit(1);
    }
    
    Ok(())
}

fn convert_single_file(
    device: &Device,
    config: &Config, 
    input_path: &str,
    output_dir: &str,
    verbose: bool
) -> Result<String, Box<dyn std::error::Error>> {
    // Determine output filename
    let input_filename = Path::new(input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output_path = format!("{}/{}.parquet", output_dir, input_filename);
    
    // Parse file based on device and config combination
    let data = match (device, config) {
        (Device::Li6800, Config::Standard) => {
            let parser = LiCor6800Standard::new();
            parser.parse_file(input_path)?
        }
        (Device::Li6800, Config::Fluorometer) => {
            let parser = LiCor6800Fluorometer::new();
            parser.parse_file(input_path)?
        }
        (Device::Li6800, Config::Aquatic) => {
            let parser = LiCor6800Aquatic::new();
            parser.parse_file(input_path)?
        }
        (Device::Li6800, Config::Soil) => {
            let parser = LiCor6800Soil::new();
            parser.parse_file(input_path)?
        }
        (Device::Li6400, _) => {
            return Err("LI-6400 support not yet implemented".into());
        }
    };
    
    if verbose {
        println!("  Parsed {} rows, {} columns", data.dataframe.height(), data.dataframe.width());
        println!("  Device: {} ({})", data.metadata.device_serial, data.metadata.console_version);
    }
    
    // Write to Parquet file
    use polars::prelude::*;
    let mut file = std::fs::File::create(&output_path)?;
    ParquetWriter::new(&mut file)
        .finish(&mut data.dataframe.clone())?;
    
    Ok(output_path)
}