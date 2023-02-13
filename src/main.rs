#![allow(warnings)]
mod algorithm;
mod cli;
mod core;
mod depricated;
mod rhc;
mod sha2;

use std::time::{Duration, Instant};

fn main() {
    rhc::run().unwrap_or_else(|e| e.exit());
}