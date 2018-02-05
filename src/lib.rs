extern crate chrono;
extern crate chrono_humanize;
#[macro_use]
extern crate clap;

pub mod app;
mod error;

use chrono::{DateTime, Local};
use chrono_humanize::HumanTime;
use clap::ArgMatches;
use self::error::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    eprintln!("{:#?}", matches);

    let seconds = if matches.is_present("duration") {
        matches.value_of("duration").unwrap().parse::<i64>()?
    } else {
        let start = matches
            .value_of("start")
            .map_or(Ok(Local::now()), |start| start.parse::<DateTime<Local>>())
            .map_err(Error::Chrono)?;

        let end = matches
            .value_of("end")
            .map_or(Ok(Local::now()), |end| end.parse::<DateTime<Local>>())
            .map_err(Error::Chrono)?;

        end.timestamp() - start.timestamp()
    };

    print_duration(seconds);

    Ok(())
}

fn print_duration(seconds: i64) {
    use chrono::Duration;
    println!("{:#}", HumanTime::from(Duration::seconds(seconds)));
}
