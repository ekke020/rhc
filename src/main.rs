#![allow(warnings)]
mod cli;
mod core;
mod sha2;
mod systems;

fn main() {
    let settings = cli::run();
}
