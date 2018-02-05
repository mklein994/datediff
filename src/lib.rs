extern crate chrono;
#[macro_use]
extern crate clap;

pub mod app;
mod error;

use chrono::{DateTime, Local, TimeZone};
use clap::ArgMatches;
use self::error::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    eprintln!("{:#?}", matches);

    if matches.is_present("duration") {
        let seconds = matches.value_of("duration").unwrap().parse::<usize>()?;

        print_duration(seconds)?;
    } else if matches.is_present("difference") {
        unimplemented!();
    }

    Ok(())
}

fn print_duration(s: usize) -> Result<()> {
    unimplemented!()
}
