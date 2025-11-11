//! Financial transaction parser library for YPBank.
//!
//! This library provides functionality to parse and write financial transactions
//! in multiple formats: CSV, plain text, and binary. It supports reading transactions
//! from various sources and converting between different formats.
//!
//! # Examples
//!
//! ```no_run
//! use parser::{parse, write, Format};
//! use std::fs::File;
//! use std::io::BufReader;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Parse transactions from a CSV file
//! let file = File::open("transactions.csv")?;
//! let reader = BufReader::new(file);
//! let batch = parse(reader, Format::Csv)?;
//!
//! // Write transactions to stdout in text format
//! let mut stdout = std::io::stdout();
//! write(&batch, &mut stdout, Format::Text)?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]

pub mod domain;
pub use domain::{Money, Transaction, TransactionBatch, TransactionKind};

pub mod error;
pub use error::{Error, Result};

pub mod formats;
pub use formats::Format;

use std::io::{Read, Write};

/// Parses a batch of transactions from a reader in the specified format.
///
/// This function reads transaction data from any type that implements [`Read`]
/// and parses it according to the specified [`Format`].
///
/// # Arguments
///
/// * `reader` - A reader containing transaction data
/// * `format` - The format of the input data (CSV, Text, or Binary)
///
/// # Returns
///
/// Returns a [`TransactionBatch`] containing all parsed transactions, or an [`Error`]
/// if parsing fails.
///
/// # Examples
///
/// ```no_run
/// use parser::{parse, Format};
/// use std::fs::File;
/// use std::io::BufReader;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let file = File::open("transactions.csv")?;
/// let reader = BufReader::new(file);
/// let batch = parse(reader, Format::Csv)?;
/// println!("Parsed {} transactions", batch.transactions.len());
/// # Ok(())
/// # }
/// ```
pub fn parse<R: Read>(reader: R, format: Format) -> Result<TransactionBatch> {
    match format {
        Format::Csv => formats::csv::parse_csv(reader),
        Format::Text => formats::text::parse_text(reader),
        Format::Binary => formats::binary::parse_binary(reader),
    }
}

/// Writes a batch of transactions to a writer in the specified format.
///
/// This function writes transaction data to any type that implements [`Write`]
/// in the specified [`Format`].
///
/// # Arguments
///
/// * `batch` - The transaction batch to write
/// * `writer` - A writer to output the transaction data to
/// * `format` - The desired output format (CSV, Text, or Binary)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an [`Error`] if writing fails.
///
/// # Examples
///
/// ```no_run
/// use parser::{TransactionBatch, write, Format};
/// use std::io;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let batch = TransactionBatch::default();
/// let mut stdout = io::stdout();
/// write(&batch, &mut stdout, Format::Csv)?;
/// # Ok(())
/// # }
/// ```
pub fn write<W: Write>(batch: &TransactionBatch, writer: &mut W, format: Format) -> Result<()> {
    match format {
        Format::Csv => formats::csv::write_csv(batch, writer),
        Format::Text => formats::text::write_text(batch, writer),
        Format::Binary => formats::binary::write_binary(batch, writer),
    }
}
