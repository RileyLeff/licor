% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/extendr-wrappers.R
\name{file_to_dataframe}
\alias{file_to_dataframe}
\title{Convert a LI-COR file directly to a data.frame or tibble}
\usage{
file_to_dataframe(file, format, device, config, preserve_names)
}
\arguments{
\item{file}{Path to the input LI-COR file}

\item{format}{Output format ("data.frame" or "tibble")}

\item{device}{Device type ("6800" or "6400")}

\item{config}{Measurement configuration ("standard", "fluorometer", "aquatic", "soil")}

\item{preserve_names}{Whether to preserve original LI-COR variable names (TRUE) or convert to R-friendly names (FALSE)}
}
\value{
data.frame or tibble with the converted data
}
\description{
Convert a LI-COR file directly to a data.frame or tibble
}
