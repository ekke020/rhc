#![allow(warnings)]
mod cli;
mod core;
mod sha2;
mod systems;

fn main() {
    if let Err(e) = cli::entrypoint() {
        println!("{}", e);
        std::process::exit(e.get_exit_code())
    }
}

