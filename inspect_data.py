#!/usr/bin/env python3
"""
Inspect LI-COR Parquet data converted by the licor CLI tool.

Usage:
    uv run inspect_data.py

This script reads the converted Parquet files and shows data structure,
metadata, and basic statistics.
"""
# /// script
# dependencies = [
#     "polars>=1.0.0",
#     "rich>=13.0.0",
# ]
# ///

import polars as pl
from pathlib import Path
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich.columns import Columns
import sys

console = Console()

def inspect_parquet_file(file_path: Path):
    """Inspect a single Parquet file."""
    try:
        # Read the Parquet file
        df = pl.read_parquet(file_path)
        
        console.print(f"\n[bold green]📊 File: {file_path.name}[/bold green]")
        
        # Basic info
        info_table = Table(title="Dataset Overview")
        info_table.add_column("Metric", style="cyan")
        info_table.add_column("Value", style="magenta")
        
        info_table.add_row("Rows", str(df.height))
        info_table.add_row("Columns", str(df.width))
        info_table.add_row("Memory Usage", f"{df.estimated_size('mb'):.2f} MB")
        
        console.print(info_table)
        
        # Column types summary
        type_counts = df.schema.to_python()
        type_summary = {}
        for col, dtype in type_counts.items():
            dtype_str = str(dtype)
            if dtype_str in type_summary:
                type_summary[dtype_str] += 1
            else:
                type_summary[dtype_str] = 1
        
        types_table = Table(title="Data Types")
        types_table.add_column("Type", style="cyan")
        types_table.add_column("Count", style="magenta")
        
        for dtype, count in sorted(type_summary.items()):
            types_table.add_row(dtype, str(count))
            
        console.print(types_table)
        
        # Show key gas exchange variables if they exist
        gas_exchange_vars = ["obs", "A", "E", "Ca", "Ci", "gsw", "gbw", "Tleaf", "Pa"]
        available_vars = [var for var in gas_exchange_vars if var in df.columns]
        
        if available_vars:
            console.print(f"\n[bold yellow]🌿 Gas Exchange Variables ({len(available_vars)} found)[/bold yellow]")
            
            gas_table = Table()
            gas_table.add_column("Variable", style="cyan")
            gas_table.add_column("Type", style="green")
            gas_table.add_column("Mean", style="magenta")
            gas_table.add_column("Min", style="red")
            gas_table.add_column("Max", style="red")
            gas_table.add_column("Non-null", style="blue")
            
            for var in available_vars:
                col_data = df[var]
                dtype = str(col_data.dtype)
                
                if dtype in ["Float64", "Int64"]:
                    try:
                        stats = col_data.describe()
                        mean_val = f"{col_data.mean():.3f}" if col_data.mean() is not None else "N/A"
                        min_val = f"{col_data.min():.3f}" if col_data.min() is not None else "N/A"
                        max_val = f"{col_data.max():.3f}" if col_data.max() is not None else "N/A"
                    except:
                        mean_val = min_val = max_val = "N/A"
                else:
                    mean_val = min_val = max_val = "N/A"
                
                non_null = col_data.drop_nulls().len()
                gas_table.add_row(var, dtype, mean_val, min_val, max_val, f"{non_null}/{df.height}")
            
            console.print(gas_table)
        
        # Show fluorescence variables if they exist
        fluor_vars = ["F", "Fm'", "Fo'", "PhiPS2", "ETR", "NPQ", "qP"]
        available_fluor = [var for var in fluor_vars if var in df.columns]
        
        if available_fluor:
            console.print(f"\n[bold purple]🔬 Fluorescence Variables ({len(available_fluor)} found)[/bold purple]")
            
            fluor_table = Table()
            fluor_table.add_column("Variable", style="cyan")
            fluor_table.add_column("Type", style="green") 
            fluor_table.add_column("Mean", style="magenta")
            fluor_table.add_column("Non-null", style="blue")
            
            for var in available_fluor:
                col_data = df[var]
                dtype = str(col_data.dtype)
                
                if dtype in ["Float64", "Int64"]:
                    try:
                        mean_val = f"{col_data.mean():.3f}" if col_data.mean() is not None else "N/A"
                    except:
                        mean_val = "N/A"
                else:
                    mean_val = "N/A"
                
                non_null = col_data.drop_nulls().len()
                fluor_table.add_row(var, dtype, mean_val, f"{non_null}/{df.height}")
            
            console.print(fluor_table)
        
        # Sample of first few rows for key variables
        if available_vars:
            sample_vars = available_vars[:6]  # Show first 6 variables
            console.print(f"\n[bold blue]📋 Sample Data (first 3 rows)[/bold blue]")
            sample_df = df.select(sample_vars).head(3)
            console.print(sample_df)
        
        return df
        
    except Exception as e:
        console.print(f"[bold red]❌ Error reading {file_path}: {e}[/bold red]")
        return None

def main():
    """Main inspection function."""
    console.print("[bold blue]🔍 LI-COR Data Inspector[/bold blue]")
    console.print("Analyzing converted Parquet files...\n")
    
    # Look for output directory
    output_dir = Path("output")
    if not output_dir.exists():
        console.print("[bold red]❌ No 'output' directory found. Run the CLI first:[/bold red]")
        console.print("cargo run --bin licor -- convert --device 6800 --config fluorometer --input 'example_data/*' --output output --verbose")
        sys.exit(1)
    
    # Find all Parquet files
    parquet_files = list(output_dir.glob("*.parquet"))
    
    if not parquet_files:
        console.print("[bold red]❌ No Parquet files found in output directory.[/bold red]")
        sys.exit(1)
    
    console.print(f"[green]Found {len(parquet_files)} Parquet files to analyze[/green]")
    
    # Analyze each file
    dataframes = []
    for file_path in sorted(parquet_files):
        df = inspect_parquet_file(file_path)
        if df is not None:
            dataframes.append((file_path.stem, df))
    
    # Combined analysis if multiple files
    if len(dataframes) > 1:
        console.print(f"\n[bold cyan]📈 Combined Analysis ({len(dataframes)} files)[/bold cyan]")
        
        combined_table = Table()
        combined_table.add_column("File", style="cyan")
        combined_table.add_column("Rows", style="magenta")
        combined_table.add_column("Columns", style="green")
        combined_table.add_column("Has A", style="yellow")
        combined_table.add_column("Has PhiPS2", style="purple")
        
        total_rows = 0
        for name, df in dataframes:
            has_a = "✅" if "A" in df.columns else "❌"
            has_phips2 = "✅" if "PhiPS2" in df.columns else "❌"
            combined_table.add_row(name, str(df.height), str(df.width), has_a, has_phips2)
            total_rows += df.height
        
        console.print(combined_table)
        console.print(f"\n[bold green]✨ Total observations across all files: {total_rows}[/bold green]")
    
    console.print(f"\n[bold green]🎉 Analysis complete! Your LI-COR data is ready for science! 🔬[/bold green]")

if __name__ == "__main__":
    main()