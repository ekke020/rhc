#![allow(warnings)]
mod algorithm;
mod cli;
mod core;
mod depricated;
mod rhc;
mod sha2;

use std::time::{Duration, Instant};

fn main() {
    // let instant = std::time::Instant::now();
    rhc::run().unwrap_or_else(|e| e.exit());
    // println!("{}", instant.elapsed().as_millis());
    // NO_SPECIAL_RANGE.iter().for_each(|c| print!("0x{:x}, ", *c as i32));
}

fn elapsed_time(elapsed: u64) {
    let seconds = (elapsed % 3600) % 60;
    let minutes = (elapsed % 3600 - seconds) / 60;
    let hours = (elapsed - minutes * 60 + seconds) / 3600;
    println!("H: {}, M: {}, S: {}", hours, minutes, seconds);
}