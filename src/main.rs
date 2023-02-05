#![allow(warnings)]
mod algorithm;
mod cli;
mod core;
mod depricated;
mod rhc;
mod sha2;
fn main() {
    rhc::run().unwrap_or_else(|e| e.exit());
}
