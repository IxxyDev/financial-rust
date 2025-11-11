//! Error types for the parser library.
//!
//! This module defines all error types that can occur during parsing
//! and writing of financial transaction data.

use thiserror::Error as ThisError;

/// A specialized Result type for parser operations.
///
/// This type is used throughout the library to simplify error handling.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during parsing or writing of transaction data.
#[derive(Debug, ThisError)]
pub enum Error {
    /// An I/O error occurred while reading or writing data
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The specified format is not supported
    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),

    /// Failed to parse format from string
    #[error("invalid format: {0}")]
    InvalidFormat(#[from] strum::ParseError),

    /// A parsing error occurred with details about what went wrong
    #[error("parse error in {format}: {message}")]
    Parse {
        /// The format that was being parsed
        format: &'static str,
        /// A detailed message describing the parsing error
        message: String,
    },
}

impl Error {
    /// Creates a new parse error with the specified format and message.
    ///
    /// # Arguments
    ///
    /// * `format` - The name of the format being parsed (e.g., "CSV", "Binary")
    /// * `message` - A description of what went wrong during parsing
    ///
    /// # Examples
    ///
    /// ```
    /// use parser::Error;
    ///
    /// let error = Error::parse("CSV", "Missing required field: id");
    /// ```
    pub fn parse(format: &'static str, message: impl Into<String>) -> Self {
        Self::Parse {
            format,
            message: message.into(),
        }
    }
}
