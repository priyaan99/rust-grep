use std::{env, process};

use init::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = init::run(config) {
        eprintln!("Faild to run application because of {e}");
        process::exit(1);
    }
}
