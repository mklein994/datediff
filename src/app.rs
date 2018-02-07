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
                .help("Calculate the difference between two dates. Assume today if given only one.")
                .short("d")
                .long("difference")
                .alias("diff")
                .max_values(2)
                .value_name("date"),
        )
        .arg(
            Arg::with_name("seconds")
                .help("Parse arguments as UNIX timestamps.")
                .short("s")
                .long("seconds")
                .conflicts_with("duration"),
        )
        .arg(
            Arg::with_name("rough")
                .help("Use rough accuracy. The default is to be precise.")
                .short("r")
                .long("rough"),
        )
        .subcommand(
            SubCommand::with_name("completions")
                .help("Write a tab completion script to standard output for the provided shell.")
                .arg(
                    Arg::with_name("shell")
                        .help("The shell to provide completion for.")
                        .possible_values(&Shell::variants()),
                ),
        )
}
