extern crate chrono;
#[macro_use]
extern crate clap;

pub mod app;

use clap::ArgMatches;
use std::fmt;

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

impl std::error::Error for Error {
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

type Result<T> = std::result::Result<T, Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    eprintln!("{:#?}", matches);
    Ok(())
}
