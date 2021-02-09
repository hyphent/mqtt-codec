use std::io::Error;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum DecodeError {
  ProtocolNotSupportedError,
  FormatError,
  Utf8Error(Utf8Error),
  IoError(Error)
}

impl From<Utf8Error> for DecodeError {
  fn from(error: Utf8Error) -> Self {
    DecodeError::Utf8Error(error)
  }
}

impl From<Error> for DecodeError {
  fn from(error: Error) -> Self {
    DecodeError::IoError(error)
  }
}

#[derive(Debug)]
pub enum EncodeError {
  VariableIntegerOutOfRangeError,
  FormatError,
  WebsocketError,
  IoError(Error)
}

impl From<Error> for EncodeError {
  fn from(error: Error) -> Self {
    EncodeError::IoError(error)
  }
}