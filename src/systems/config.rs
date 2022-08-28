use crate::systems::printer::print;
use clap::ArgMatches;

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
    }

    fn check_for_print_flags(&self) {
        if self.list_all {
            print::available_algorithms();
            std::process::exit(1);
        }
    }

    pub fn get_algorithm_type(&self) -> &Option<String> {
        &self.algorithm_type
    }

    pub fn get_hash(&self) -> &str {
        match &self.hash {
            Some(hash) => return &hash,
            None => {
                println!("No hash was supplied, (-h, --help for options)");
                std::process::exit(1);
            }
        }
    }

    pub fn get_length(&self) -> Option<usize> {
        self.length
    }
}

impl From<ArgMatches> for Config {
    fn from(m: ArgMatches) -> Self {
        exit_if_empty(&m);
        Config {
            length: m.get_one::<usize>("length").cloned(),
            algorithm_type: m.get_one::<String>("type").cloned(),
            list_all: m.is_present("all"),
            hash: m.get_one::<String>("input").cloned(),
        }
    }
}

fn exit_if_empty(m: &ArgMatches) {
    if !m.args_present() {
        println!("Insufficient arguments, (-h, --help for options)");
        std::process::exit(1);
    }
}
