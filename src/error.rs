use chrono;
use std::error;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum Error {
    Chrono(chrono::ParseError),
    ParseInt(num::ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Chrono(ref err) => write!(f, "chrono error: {}", err),
            Error::ParseInt(ref err) => write!(f, "parse int error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Chrono(ref err) => err.description(),
            Error::ParseInt(ref err) => err.description(),
        }
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Self {
        Error::Chrono(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}
