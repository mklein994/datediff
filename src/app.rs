use clap::{App, Arg, Shell, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("duration").multiple(true))
        .arg(
            Arg::with_name("difference")
                .short("d")
                .long("difference")
                .alias("diff")
                .max_values(2),
        )
        .arg(Arg::with_name("rough").short("r").long("rough"))
        .subcommand(
            SubCommand::with_name("completions")
                .arg(Arg::with_name("shell").possible_values(&Shell::variants())),
        )
}
