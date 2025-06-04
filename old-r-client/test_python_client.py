#!/usr/bin/env python3
"""Test script to verify the core functionality works via Python client."""

import sys
import os
sys.path.append('../python-client')

try:
    import licor_client
    print("✅ Python client imported successfully")
    
    # Test file path
    sample_file = "inst/extdata/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1"
    
    if os.path.exists(sample_file):
        print(f"✅ Sample file exists: {sample_file}")
        
        # Test conversion
        try:
            df = licor_client.file_to_dataframe(
                file=sample_file,
                format="polars",
                device="6800", 
                config="fluorometer"
            )
            print(f"✅ Conversion successful: {df.shape[0]} rows x {df.shape[1]} columns")
            print(f"   First few column names: {df.columns[:5]}")
            
        except Exception as e:
            print(f"❌ Conversion failed: {e}")
    else:
        print(f"❌ Sample file not found: {sample_file}")
        
except ImportError as e:
    print(f"❌ Python client import failed: {e}")
    print("   Make sure Python client is built: cd ../python-client && maturin develop")
except Exception as e:
    print(f"❌ Unexpected error: {e}")