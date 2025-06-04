"""Type stubs for licor_client."""

from typing import Any, Literal, Union

def convert(
    file: str,
    output: str, 
    device: Literal["6800", "6400"],
    config: Literal["standard", "fluorometer", "aquatic", "soil"]
) -> None:
    """Convert a LI-COR file to Parquet format.
    
    Args:
        file: Path to the input LI-COR file
        output: Path for the output Parquet file  
        device: Device type ("6800" or "6400")
        config: Measurement configuration ("standard", "fluorometer", "aquatic", "soil")
        
    Raises:
        ValueError: Invalid device/config combination or malformed data
        IOError: File read/write errors
        RuntimeError: Other parsing errors
    """
    ...

def file_to_dataframe(
    file: str,
    format: Literal["polars", "pandas"],
    device: Literal["6800", "6400"], 
    config: Literal["standard", "fluorometer", "aquatic", "soil"]
) -> Any:
    """Convert a LI-COR file directly to a DataFrame.
    
    Args:
        file: Path to the input LI-COR file
        format: Output format ("polars" or "pandas")
        device: Device type ("6800" or "6400") 
        config: Measurement configuration ("standard", "fluorometer", "aquatic", "soil")
        
    Returns:
        DataFrame in the requested format
        
    Raises:
        ValueError: Invalid device/config combination, unsupported format, or malformed data
        IOError: File read errors
        RuntimeError: Missing optional dependencies or other parsing errors
    """
    ...