use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    run(&config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
