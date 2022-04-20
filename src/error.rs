use std::io;

#[derive(Debug)]
pub enum Error {
  Io(io::Error),
  Regex(regex::Error)
}

impl From<io::Error> for Error {
  fn from(value: io::Error) -> Self {
    Error::Io(value)
  }
}

impl From<regex::Error> for Error {
  fn from(value: regex::Error) -> Self {
    Error::Regex(value)
  }
}
