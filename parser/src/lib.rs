pub mod domain;
pub use domain::{Money, Transaction, TransactionBatch, TransactionKind};

pub mod error;
pub use error::{Error, Result};

pub mod formats;
pub use formats::Format;

use std::io::{Read, Write};

pub fn parse<R: Read>(reader: R, format: Format) -> Result<TransactionBatch> {
    match format {
        Format::Csv => formats::csv::parse_csv(reader),
        Format::Text => formats::text::parse_text(reader),
        Format::Binary => formats::binary::parse_binary(reader),
    }
}

pub fn write<W: Write>(batch: &TransactionBatch, writer: &mut W, format: Format) -> Result<()> {
    match format {
        Format::Csv => formats::csv::write_csv(batch, writer),
        Format::Text => formats::text::write_text(batch, writer),
        Format::Binary => formats::binary::write_binary(batch, writer),
    }
}
