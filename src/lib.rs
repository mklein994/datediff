extern crate chrono;
extern crate chrono_humanize;
#[macro_use]
extern crate clap;

pub mod app;
mod error;

use chrono::{DateTime, Local, TimeZone};
use chrono_humanize::HumanTime;
use clap::ArgMatches;
use self::error::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    eprintln!("{:#?}", matches);

    if matches.is_present("duration") {
        let seconds = matches.value_of("duration").unwrap().parse::<i64>()?;

        print_duration(seconds)?;
    } else {
        if matches.is_present("start") {
            let start = matches
                .value_of("start")
                .map(|s| s.parse::<DateTime<Local>>()?)
                .unwrap_or_else(|| Local::now());

            let end = <DateTime<Local>>::parse(matches.value_of("end")
                .unwrap_or_else(|| Local::now());
                (|e| e.parse::<DateTime<Local>>()?)

            print_duration(end.timestamp() - start.timestamp())?;
        }
    }

    Ok(())
}

fn print_duration(seconds: i64) -> Result<()> {
    use chrono::Duration;
    println!("{}", HumanTime::from(Duration::seconds(seconds)));
    Ok(())
}
