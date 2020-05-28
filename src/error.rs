#![feature(try_trait)]

use std::fmt::{self, Debug, Formatter, Display};
use std::io;
use std::error;
use std::option;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    FmtError(fmt::Error),
    NoneError(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::IoError(ref e) => std::fmt::Display::fmt(&e, f),
            Error::FmtError(ref e) => std::fmt::Display::fmt(&e, f),
            Error::NoneError(ref e) => std::fmt::Display::fmt(&e, f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::IoError(ref e) => Some(e),
            Error::FmtError(ref e) => Some(e),
            Error::NoneError(ref e) => Some(Error::NoneError("None".to_string())),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::FmtError(e)
    }
}
