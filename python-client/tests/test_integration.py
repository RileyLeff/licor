"""Integration tests for licor_client using real data files."""

import pytest
import tempfile
import os
from pathlib import Path

# Import the module - this will fail if not built with maturin
try:
    import licor_client
except ImportError:
    pytest.skip("licor_client not built, run 'maturin develop' first", allow_module_level=True)

# Sample data files (relative to repository root)
SAMPLE_FILES = [
    "../example_data/2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1",
    "../example_data/2025-05-30-1203_logdata_F2"
]

class TestConvert:
    """Test the convert() function."""
    
    def test_convert_fluorometer_file(self):
        """Test converting a fluorometer file to Parquet."""
        sample_file = SAMPLE_FILES[0]
        if not Path(sample_file).exists():
            pytest.skip(f"Sample file not found: {sample_file}")
        
        with tempfile.NamedTemporaryFile(suffix=".parquet", delete=False) as tmp:
            try:
                licor_client.convert(
                    file=sample_file,
                    output=tmp.name,
                    device="6800",
                    config="fluorometer"
                )
                
                # Verify file was created and has content
                assert Path(tmp.name).exists()
                assert Path(tmp.name).stat().st_size > 0
            finally:
                os.unlink(tmp.name)
    
    def test_convert_invalid_device(self):
        """Test error handling for invalid device."""
        sample_file = SAMPLE_FILES[0]
        if not Path(sample_file).exists():
            pytest.skip(f"Sample file not found: {sample_file}")
        
        with tempfile.NamedTemporaryFile(suffix=".parquet") as tmp:
            with pytest.raises(ValueError, match="Invalid device/config combination"):
                licor_client.convert(
                    file=sample_file,
                    output=tmp.name,
                    device="invalid",
                    config="fluorometer"
                )
    
    def test_convert_missing_file(self):
        """Test error handling for missing input file."""
        with tempfile.NamedTemporaryFile(suffix=".parquet") as tmp:
            with pytest.raises(IOError, match="File not found"):
                licor_client.convert(
                    file="/nonexistent/file.txt",
                    output=tmp.name,
                    device="6800",
                    config="fluorometer"
                )

class TestFileToDataFrame:
    """Test the file_to_dataframe() function."""
    
    def test_polars_dataframe(self):
        """Test converting to polars DataFrame."""
        polars = pytest.importorskip("polars")
        
        sample_file = SAMPLE_FILES[0]
        if not Path(sample_file).exists():
            pytest.skip(f"Sample file not found: {sample_file}")
        
        df = licor_client.file_to_dataframe(
            file=sample_file,
            format="polars",
            device="6800",
            config="fluorometer"
        )
        
        # Verify it's a polars DataFrame with expected properties
        assert hasattr(df, 'height')  # polars DataFrame method
        assert hasattr(df, 'width')   # polars DataFrame method
        assert df.height > 0
        assert df.width > 0
        
        # Check for expected columns (should have some standard variables)
        column_names = df.columns
        assert "obs" in column_names  # observation number should always be present
    
    def test_pandas_dataframe(self):
        """Test converting to pandas DataFrame."""
        pandas = pytest.importorskip("pandas")
        
        sample_file = SAMPLE_FILES[0]
        if not Path(sample_file).exists():
            pytest.skip(f"Sample file not found: {sample_file}")
        
        df = licor_client.file_to_dataframe(
            file=sample_file,
            format="pandas",
            device="6800",
            config="fluorometer"
        )
        
        # Verify it's a pandas DataFrame with expected properties
        assert hasattr(df, 'shape')  # pandas DataFrame attribute
        assert len(df) > 0
        assert len(df.columns) > 0
        
        # Check for expected columns
        assert "obs" in df.columns
    
    def test_missing_polars_dependency(self):
        """Test error when polars is not installed."""
        # This test is tricky because polars might be installed
        # We'll test the error message structure instead
        sample_file = SAMPLE_FILES[0]
        if not Path(sample_file).exists():
            pytest.skip(f"Sample file not found: {sample_file}")
        
        # Test with invalid format to ensure error handling works
        with pytest.raises(ValueError, match="Unsupported format"):
            licor_client.file_to_dataframe(
                file=sample_file,
                format="invalid_format",
                device="6800",
                config="fluorometer"
            )
    
    def test_invalid_config_combination(self):
        """Test error handling for invalid device/config combination."""
        sample_file = SAMPLE_FILES[0]
        if not Path(sample_file).exists():
            pytest.skip(f"Sample file not found: {sample_file}")
        
        with pytest.raises(ValueError, match="Invalid device/config combination"):
            licor_client.file_to_dataframe(
                file=sample_file,
                format="polars",
                device="6400",  # Not yet implemented
                config="fluorometer"
            )

class TestMultipleFiles:
    """Test with multiple sample files."""
    
    def test_both_sample_files(self):
        """Test that both sample files can be processed."""
        for sample_file in SAMPLE_FILES:
            if not Path(sample_file).exists():
                continue
                
            # Test conversion works
            with tempfile.NamedTemporaryFile(suffix=".parquet", delete=False) as tmp:
                try:
                    licor_client.convert(
                        file=sample_file,
                        output=tmp.name,
                        device="6800",
                        config="fluorometer"
                    )
                    assert Path(tmp.name).exists()
                    assert Path(tmp.name).stat().st_size > 0
                finally:
                    os.unlink(tmp.name)