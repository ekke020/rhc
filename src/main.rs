#![allow(warnings)]
mod cli;
mod core;
mod sha2;
mod depricated;

fn main() {
    // let settings = cli::run();
    let settings = cli::settings::GlobalSettings::new();
    core::run(settings);
}
