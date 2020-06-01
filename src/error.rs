use std::{
    io, error,
    fmt:: {
        self, Debug, Formatter, Display
    }
};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    FmtError(fmt::Error),
    NoneError(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Error::IoError(ref e) => fmt::Display::fmt(&e, f),
            Error::FmtError(ref e) => fmt::Display::fmt(&e, f),
            Error::NoneError(s) => write!(f, "{}", s)
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Error::IoError(ref e) => Some(e),
            Error::FmtError(ref e) => Some(e),
            Error::NoneError(_s) => None
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

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::NoneError(e)
    }
}
