use crate::prelude::*;
use std::env;

struct Config {
    hash: String,
    hash_type: hasher::HashType,
    password_length: usize,
}

mod cli {
    use super::*;
    enum Flags {
        Help,
        Length,
        Type,
        All,
    }
    
    fn parse_input(index: usize, input: &str) {
        let help = env::args()
            .enumerate()
            .find(|h| h.1.eq("-h") || h.1.eq("--help"));
    }
}

pub fn take_arguments() {
    let arguments = env::args().collect::<Vec<String>>();
    let help = env::args()
        .enumerate()
        .find(|h| h.1.eq("-h") || h.1.eq("--help"));
    // arguments.iter().skip(1).for_each(|f| println!("{}", f));
}

fn parse_input(input: &str) {
    let help = env::args()
        .enumerate()
        .find(|h| h.1.eq("-h") || h.1.eq("--help"));
}

fn parse_config(args: &[String]) -> Result<Config, String> {
    match args.len() {
        1 => return Err(String::from("Need at least two arguments")),
        _ => return Err(String::from("Invalid configuration")),
    }
}
