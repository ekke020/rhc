#![allow(warnings)]
mod algorithm;
mod cli;
mod core;
mod depricated;
mod rhc;
mod sha2;

use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    rhc::run().unwrap_or_else(|e| e.exit());
    let duration = start.elapsed();
    elapsed_time(duration.as_secs());
}

fn elapsed_time(elapsed: u64) {
    let seconds = (elapsed % 3600) % 60;
    let minutes = (elapsed % 3600 - seconds) / 60;
    let hours = (elapsed - minutes * 60 + seconds) / 3600;
    println!("H: {}, M: {}, S: {}", hours, minutes, seconds);
}