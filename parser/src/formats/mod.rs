pub mod binary;
pub mod csv;
pub mod text;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Csv,
    Text,
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
    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Csv => "csv",
            Format::Text => "text",
            Format::Binary => "binary",
        }
    }
}
