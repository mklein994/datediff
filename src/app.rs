use clap::{App, Arg, ArgMatches};

pub fn build_cli<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("duration")
                .short("d")
                .long("duration")
                .takes_value(true),
        )
        .get_matches()
}
