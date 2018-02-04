use chrono;
use ::std::fmt;
use ::std::error;

#[derive(Debug)]
pub enum Error {
    Chrono(chrono::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Chrono(ref err) => write!(f, "chrono error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Chrono(ref err) => err.description(),
        }
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Self {
        Error::Chrono(err)
    }
}

