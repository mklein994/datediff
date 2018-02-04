extern crate datediff;

fn main() {
    let matches = datediff::app::build_cli();

    if let Err(err) = datediff::run(matches) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
