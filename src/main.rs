#![allow(warnings)]
mod cli;
mod core;
mod sha2;
mod depricated;

fn main() {
    let settings = cli::run();
    core::run(settings);
}
