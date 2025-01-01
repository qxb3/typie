mod config;
mod cli;
mod ui;
mod typie;
mod utils;

use std::process;
use typie::Typie;

fn main() {
    let config = match cli::run() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("[ERROR] {err}");
            process::exit(1);
        }
    };

    let mut typie = Typie::new(&config);
    if let Err(err) = typie.run() {
        eprintln!("[ERROR] {err}");
        process::exit(1);
    }
}
