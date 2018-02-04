extern crate datediff;

use std::env;

fn main() {
    let args = env::args();

    if let Err(err) = datediff::run(args) {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}
