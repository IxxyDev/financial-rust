//! Error types for the parser library.
//!
//! This module defines all error types that can occur during parsing
//! and writing of financial transaction data.

use std::error;
use std::fmt;

/// A specialized Result type for parser operations.
///
/// This type is used throughout the library to simplify error handling.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during parsing or writing of transaction data.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred while reading or writing data
    Io(std::io::Error),
    /// The specified format is not supported
    UnsupportedFormat(&'static str),
    /// A parsing error occurred with details about what went wrong
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "I/O error: {err}"),
            Error::UnsupportedFormat(fmt_name) => {
                write!(f, "unsupported format: {fmt_name}")
            }
            Error::Parse { format, message } => {
                write!(f, "parse error in {format}: {message}")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::UnsupportedFormat(_) | Error::Parse { .. } => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
