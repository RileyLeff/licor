# licor-client

Python client for LI-COR instrument data conversion, providing fast and type-safe conversion of LI-COR data files to analysis-ready formats.

## Installation

```bash
# Basic installation
uv add licor-client

# With polars support
uv add licor-client[polars]

# With pandas support  
uv add licor-client[pandas]

# With both DataFrame libraries
uv add licor-client[dataframes]
```

## Usage

### Convert to Parquet file

```python
import licor_client

# Convert LI-6800 fluorometer data to Parquet
licor_client.convert(
    file="data.txt",
    output="data.parquet", 
    device="6800",
    config="fluorometer"
)
```

### Convert to DataFrame

```python
import licor_client

# Get as polars DataFrame
df = licor_client.file_to_dataframe(
    file="data.txt",
    format="polars",
    device="6800", 
    config="fluorometer"
)

# Get as pandas DataFrame
df = licor_client.file_to_dataframe(
    file="data.txt",
    format="pandas",
    device="6800",
    config="standard"
)
```

## Supported Devices and Configurations

- **Devices**: `"6800"` (LI-6800), `"6400"` (planned)
- **Configurations**: 
  - `"standard"` - Basic gas exchange measurements
  - `"fluorometer"` - Gas exchange + chlorophyll fluorescence  
  - `"aquatic"` - Aquatic chamber measurements (planned)
  - `"soil"` - Soil respiration measurements (planned)

## Requirements

- Python 3.8+
- Optional: `polars>=0.20.0` for polars DataFrame support
- Optional: `pandas>=1.0.0` + `pyarrow>=10.0.0` for pandas DataFrame support