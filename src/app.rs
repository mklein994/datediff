use clap::{App, Arg, ArgMatches};

pub fn build_cli<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("duration")
                .short("d")
                .long("duration")
                .takes_value(true)
                .conflicts_with("difference"),
        )
        .arg(
            Arg::with_name("begin")
                .short("b")
                .long("begin")
                .aliases(&["start", "s"])
                .takes_value(true)
                .requires("end"),
        )
        .arg(
            Arg::with_name("end")
                .short("e")
                .long("end")
                .aliases(&["finish", "f"])
                .takes_value(true),
        )
        .get_matches()
}
