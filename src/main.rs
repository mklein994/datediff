extern crate datediff;
#[macro_use]
extern crate clap;

use datediff::app;
use clap::Shell;

fn main() {
    let matches = datediff::app::build_cli().get_matches();

    if let ("completions", Some(completion_matches)) = matches.subcommand() {
        let shell = completion_matches
            .value_of("shell")
            .and_then(|s| s.parse::<Shell>().ok())
            .expect("couldn't parse shell name");

        app::build_cli().gen_completions_to(crate_name!(), shell, &mut std::io::stdout());
    } else {
        if let Err(err) = datediff::run(&matches) {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
