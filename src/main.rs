use std::process;

mod config;
mod cli;

fn main() {
    let config = match cli::run() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("[ERROR] {err}");
            process::exit(1);
        }
    };

    println!("{:?}", config);
}
