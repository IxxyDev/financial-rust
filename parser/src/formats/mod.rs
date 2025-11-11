//! Format parsers and writers for transaction data.
//!
//! This module provides functionality to read and write transaction data
//! in various formats. Each format has its own submodule with specialized
//! parsing and writing functions.

/// Binary format parser and writer.
///
/// This module provides functions to parse and write transaction data
/// in a compact binary format.
pub mod binary;

/// CSV format parser and writer.
///
/// This module provides functions to parse and write transaction data
/// in comma-separated values format.
pub mod csv;

/// Plain text format parser and writer.
///
/// This module provides functions to parse and write transaction data
/// in a human-readable plain text format.
pub mod text;

use std::str::FromStr;

/// Supported transaction file formats.
///
/// This enum represents all formats that can be used to parse
/// and write transaction data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Comma-separated values format
    Csv,
    /// Human-readable plain text format
    Text,
    /// Compact binary format
    Binary,
}

impl FromStr for Format {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(Format::Csv),
            "text" | "txt" => Ok(Format::Text),
            "binary" | "bin" => Ok(Format::Binary),
            _ => Err(crate::Error::UnsupportedFormat("unknown format")),
        }
    }
}

impl Format {
    /// Returns the string representation of the format.
    ///
    /// # Examples
    ///
    /// ```
    /// use parser::Format;
    ///
    /// assert_eq!(Format::Csv.as_str(), "csv");
    /// assert_eq!(Format::Binary.as_str(), "binary");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Csv => "csv",
            Format::Text => "text",
            Format::Binary => "binary",
        }
    }
}
