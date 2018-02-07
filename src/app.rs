use clap::{App, Arg, Shell, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("duration")
                .help("Convert seconds to a human readable format.")
                .multiple(true),
        )
        .arg(
            Arg::with_name("difference")
                .short("d")
                .long("difference")
                .help(
                    "Calculate the difference between two dates. Assuming today if given only one.",
                )
                .alias("diff")
                .value_name("date")
                .max_values(2),
        )
        .arg(
            Arg::with_name("seconds")
                .short("s")
                .long("seconds")
                .help("Parse arguments as UNIX timestamps.")
                .conflicts_with("duration"),
        )
        .arg(
            Arg::with_name("rough")
                .short("r")
                .long("rough")
                .help("Use rough accuracy. The default is to be precise."),
        )
        .subcommand(
            SubCommand::with_name("completions")
                .help("write a tab completion script to standard output for the provided shell.")
                .arg(Arg::with_name("shell").possible_values(&Shell::variants())),
        )
}
