use crate::prelude::*;
use std::env;

struct Config {
  hash: String,
  hash_type: hasher::HashType,
  password_length: usize,
}

pub fn take_arguments() {
  let arguments = env::args().collect::<Vec<String>>();
  let help = env::var("-h").is_ok();
  println!("{}", help);
  arguments.iter().skip(1).for_each(|f| println!("{}", f));
}

fn parse_config(args: &[String]) -> Result<Config, String> {
  match args.len() {
    1 => return Err(String::from("Need at least two arguments")),
    _ => return Err(String::from("Invalid configuration")),
  }
}