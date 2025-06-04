#' Convert LI-COR File to Parquet Format
#' 
#' Converts a LI-COR instrument data file (particularly LI-6800 Portable 
#' Photosynthesis System) from its proprietary format to analysis-ready 
#' Parquet format with rich metadata preservation.
#' 
#' @param file Character string. Path to the input LI-COR file.
#' @param output Character string. Path for the output Parquet file.
#' @param device Character string. Device type. Currently supports "6800".
#' @param config Character string. Measurement configuration. One of: 
#'   "standard", "fluorometer", "aquatic", "soil".
#' 
#' @details
#' This function provides type-safe conversion with compile-time validation
#' of device/config combinations. All metadata from LI-COR headers is 
#' preserved in the Parquet output, including device serial numbers, 
#' calibration data, and variable descriptions.
#' 
#' @examples
#' \dontrun{
#' # Convert fluorometer data
#' convert(
#'   file = system.file("extdata", 
#'     "2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1", 
#'     package = "licor_client"),
#'   output = "fluorometer_data.parquet",
#'   device = "6800",
#'   config = "fluorometer"
#' )
#' }
#' 
#' @export
convert <- function(file, output, device, config) {
  .Call(wrap__convert, file, output, device, config)
}

#' Convert LI-COR File to R Data Frame
#' 
#' Converts a LI-COR instrument data file directly to an R data.frame or tibble
#' with proper data types and optional column name cleaning.
#' 
#' @param file Character string. Path to the input LI-COR file.
#' @param format Character string. Output format: "data.frame" or "tibble".
#' @param device Character string. Device type. Currently supports "6800".
#' @param config Character string. Measurement configuration. One of: 
#'   "standard", "fluorometer", "aquatic", "soil".
#' @param preserve_names Logical. If TRUE (default), preserves original 
#'   LI-COR variable names (which may require backticks in R). If FALSE, 
#'   converts to R-friendly names (e.g., delta_co2, fv_fm).
#' 
#' @return A data.frame or tibble containing the converted LI-COR data with
#'   appropriate data types (numeric, character, logical) based on variable
#'   definitions.
#' 
#' @details
#' This function handles the complex LI-COR data format including:
#' \itemize{
#'   \item Mixed data types with graceful fallback to character when 
#'     type conversion fails
#'   \item Duplicate column handling with automatic renaming
#'   \item Rich metadata preservation from instrument headers
#'   \item Scientific notation and special characters in variable names
#' }
#' 
#' When \code{preserve_names = FALSE}, problematic characters are converted:
#' \itemize{
#'   \item Greek letters: Δ becomes "delta_"
#'   \item Math symbols: / becomes "_per_", % becomes "_pct"
#'   \item Special chars: @ becomes "_at_", : becomes "_"
#'   \item Names starting with numbers get "X" prefix
#' }
#' 
#' @examples
#' \dontrun{
#' # Convert to data.frame with original names
#' df <- file_to_dataframe(
#'   file = system.file("extdata", 
#'     "2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1", 
#'     package = "licor_client"),
#'   format = "data.frame",
#'   device = "6800", 
#'   config = "fluorometer",
#'   preserve_names = TRUE
#' )
#' 
#' # Access problematic column names with backticks
#' head(df$`ΔCO2`)
#' head(df$`Fv/Fm`)
#' 
#' # Convert with R-friendly names
#' df_clean <- file_to_dataframe(
#'   file = system.file("extdata", 
#'     "2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1", 
#'     package = "licor_client"),
#'   format = "data.frame",
#'   device = "6800", 
#'   config = "fluorometer", 
#'   preserve_names = FALSE
#' )
#' 
#' # Access with standard R syntax
#' head(df_clean$delta_co2)
#' head(df_clean$fv_per_fm)
#' 
#' # Convert to tibble (requires tibble package)
#' if (requireNamespace("tibble", quietly = TRUE)) {
#'   tbl <- file_to_dataframe(
#'     file = system.file("extdata", 
#'       "2025-05-30-0948_logdata_flr_kinetics_and_gas_ex1", 
#'       package = "licor_client"),
#'     format = "tibble",
#'     device = "6800",
#'     config = "fluorometer"
#'   )
#'   print(tbl)
#' }
#' }
#' 
#' @export
file_to_dataframe <- function(file, format = "data.frame", device, config, preserve_names = TRUE) {
  .Call(wrap__file_to_dataframe, file, format, device, config, preserve_names)
}