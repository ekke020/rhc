mod errors;
use std::collections::VecDeque;
use std::env;
mod argument;
mod entry;
mod flag;

use argument::info;

pub fn entrypoint() {
    let values = entry::entry();
    let test = entry::parse_args(values);
    match test {
        Ok(flags) => flags.iter().for_each(|f| println!("{:?}", f)),
        Err(e) => e.exit(0x40),
    }

    for arg in info::ARGUMENTS {
        println!("{}", argument::describe(arg))
    }
}

