use clap::ArgMatches;
use crate::systems::printer::print;

#[derive(Debug)]
pub struct Config {
  length: Option<usize>,
  algorithm_type: Option<String>,
  hash: Option<String>,
  list_all: bool,
}

impl Config {

  pub fn scan(&self) {
    self.check_for_print_flags();
    self.exit_if_hash_is_unavailable();
  }
  
  fn check_for_print_flags(&self) {
      if self.list_all {
          print::available_algorithms();
          std::process::exit(1);
      }
  }

  fn exit_if_hash_is_unavailable(&self) {
      if self.hash.is_none() {
          println!("No hash was supplied, (-h, --help for options)");
          std::process::exit(1);
      }
  }
}

impl From<ArgMatches> for Config {
  fn from(m: ArgMatches) -> Self {
      exit_if_empty(&m);
      Config {
          length: m.get_one::<usize>("length").cloned(),
          algorithm_type: m.get_one::<String>("type").cloned(),
          list_all: m.is_present("all"),
          hash: m.get_one::<String>("hash").cloned(),
      }
  }
}

fn exit_if_empty(m: &ArgMatches) {
  if !m.args_present() {
      println!("Insufficient arguments, (-h, --help for options)");
      std::process::exit(1);
  }
}