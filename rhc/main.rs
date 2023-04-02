#![allow(warnings)]
mod central;
mod cli;
mod core;
mod error;
mod rhc;
mod settings;

fn main() {
    rhc::run().unwrap_or_else(|e| e.exit());
}
