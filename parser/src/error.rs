use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Io(std::io::Error),
  UnsupportedFormat(&'static str),
  Parse {
    format: &'static str,
    message: String,
  }
}

impl Error {
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