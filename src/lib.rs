extern crate chrono;
extern crate chrono_humanize;
#[macro_use]
extern crate clap;

pub mod app;
mod error;

use chrono::{DateTime, Local, TimeZone};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use clap::ArgMatches;
use self::error::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn run(matches: &ArgMatches) -> Result<()> {
    let accuracy = get_accuracy(matches.is_present("rough"));

    if matches.is_present("duration") {
        for i in matches.values_of("duration").unwrap() {
            print_duration(i.parse::<i64>()?, accuracy);
        }
    } else if matches.is_present("difference") {
        let (start, end) = if matches.values_of("difference").unwrap().count() < 2 {
            let time_string = matches.value_of("difference").unwrap();
            (
                Local::now(),
                if matches.is_present("seconds") {
                    Local.timestamp(time_string.parse::<i64>()?, 0)
                } else {
                    matches
                        .value_of("difference")
                        .unwrap()
                        .parse::<DateTime<Local>>()?
                },
            )
        } else {
            let mut times = matches.values_of("difference").unwrap().map(|t| {
                if matches.is_present("seconds") {
                    Ok(Local.timestamp(t.parse::<i64>()?, 0))
                } else {
                    t.parse::<DateTime<Local>>().map_err(Error::Chrono)
                }
            });

            (times.next().unwrap()?, times.next().unwrap()?)
        };

        let seconds = end.timestamp() - start.timestamp();
        print_duration(seconds, accuracy);
    };

    Ok(())
}

fn print_duration(seconds: i64, accuracy: Accuracy) {
    use chrono::Duration;
    println!(
        "{}",
        HumanTime::from(Duration::seconds(seconds)).to_text_en(accuracy, Tense::Present)
    );
}

fn get_accuracy(is_rough: bool) -> Accuracy {
    if is_rough {
        Accuracy::Rough
    } else {
        Accuracy::Precise
    }
}
