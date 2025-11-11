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

/// Supported transaction file formats.
///
/// This enum represents all formats that can be used to parse
/// and write transaction data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(strum::EnumString, strum::Display, strum::AsRefStr)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[strum(serialize_all = "lowercase")]
pub enum Format {
    /// Comma-separated values format
    #[cfg_attr(feature = "cli", value(name = "csv"))]
    Csv,

    /// Human-readable plain text format
    #[cfg_attr(feature = "cli", value(name = "text", alias = "txt"))]
    #[strum(serialize = "text")]
    #[strum(serialize = "txt")]
    Text,

    /// Compact binary format
    #[cfg_attr(feature = "cli", value(name = "binary", alias = "bin"))]
    #[strum(serialize = "binary")]
    #[strum(serialize = "bin")]
    Binary,
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
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}
