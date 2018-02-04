extern crate chrono;
#[macro_use]
extern crate clap;

pub mod app;
mod error;

use clap::ArgMatches;
use self::error::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    eprintln!("{:#?}", matches);
    Ok(())
}
