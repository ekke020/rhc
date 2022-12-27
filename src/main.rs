#![allow(warnings)]
mod core;
mod sha2;
mod systems;
mod cli;
use std::convert::TryInto;
use std::env;

use crate::systems::printer::print;
fn main() {
    cli::entrypoint();
}