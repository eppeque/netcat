use std::{env, process};

use netcat::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = netcat::run(config) {
        eprintln!("{e}");
        process::exit(1);
    };
}
