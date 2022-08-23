mod systems;
#[macro_use]
extern crate lazy_static;
pub mod prelude {
    pub use crate::systems::cracker;
    pub use crate::systems::filereader;
    pub use crate::systems::hasher;
    pub use crate::systems::input;
    pub use crate::systems::macros;
    pub use crate::systems::password_info as PW;
    pub use crate::systems::password_matcher as matcher;
    pub use crate::systems::printer::print;
    pub const CRACK_TEST: &str = "99fb2f48c6af4761f904fc85f95eb56190e5d40b1f44ec3a9c1fa319";
    lazy_static! {
        pub static ref MATCHER_INFO: matcher::MatcherInfo = matcher::MatcherInfo::new(CRACK_TEST);
        pub static ref START_TIME: std::time::SystemTime = std::time::SystemTime::now();
    }
}
use clap::value_parser;
use clap::ArgMatches;
use prelude::*;
use std::env::args;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

use clap::Arg;
use clap::Command;

#[derive(Debug)]
struct Config {
    length: Option<usize>,
    algorithm_type: Option<String>,
    hash: Option<String>,
    list_all: bool,
}

impl Config {
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
fn main() {
    // input::take_arguments();

    let args = match_input();
    let config = Config::from(args);

    let result = hash::get_possible_algorithm(CRACK_TEST).unwrap();
    let pw_info = PW::PasswordInfo::new(vec![result.0, result.1]);
    pw_info
        .get_possible_hash_types()
        .iter()
        .for_each(|f| println!("{f}"));

    // let (tx, rx) = mpsc::channel();
    // for i in 0..95 {
    //     let start_char = (32 + i as u8) as char;
    //     spin_up_thread(tx.clone(), start_char, i);
    // }
    // let received = rx.recv().unwrap();
    // received.print();
}

fn match_input() -> ArgMatches {
    Command::new("rhc")
        .about("A password cracking tool that utilizes brute force to crack passwords.")
        .arg(
            Arg::new("length")
                .required(false)
                .takes_value(true)
                .value_parser(value_parser!(usize))
                .long("length")
                .short('l')
                .help("The length of the hashed value."),
        )
        .arg(
            Arg::new("type")
                .required(false)
                .takes_value(true)
                .value_parser(value_parser!(String))
                .long("type")
                .short('t')
                .help("Specifies the algorithm used to generate the hash"),
        )
        .arg(
            Arg::new("all")
                .required(false)
                .long("all")
                .short('a')
                .help("List available algorithms"),
        )
        .arg(
            Arg::new("input")
                .required(false)
                .takes_value(true)
                .long("input")
                .short('i')
                .value_parser(value_parser!(String))
                .help("the supplied hash"),
        )
        .get_matches()
}

mod hash {
    use crate::prelude::*;
    pub fn get_possible_algorithm(
        input: &str,
    ) -> Result<(hasher::HashType, hasher::HashType), String> {
        let hash;
        match input.as_bytes().len() * 4 {
            224 => {
                hash = (
                    hasher::HashType::Sha224(input.to_owned()),
                    hasher::HashType::Sha512_224(input.to_owned()),
                )
            }
            256 => {
                hash = (
                    hasher::HashType::Sha256(input.to_owned()),
                    hasher::HashType::Sha512_256(input.to_owned()),
                )
            }
            384 => {
                hash = (
                    hasher::HashType::Sha384(input.to_owned()),
                    hasher::HashType::Empty,
                )
            }
            512 => {
                hash = (
                    hasher::HashType::Sha512(input.to_owned()),
                    hasher::HashType::Empty,
                )
            }
            _ => return Err("Unable to detect algorithm...".to_string()),
        }
        Ok(hash)
    }
}

fn spin_up_thread(tx: Sender<PW::PasswordInfo>, star_char: char, num: i32) {
    thread::spawn(move || {
        let mut pw_length = 0;
        loop {
            println!("Thread: {} is on length: {}", num, pw_length + 1);
            let password_info = cracker::crack(pw_length, &star_char);
            match password_info {
                Some(t) => {
                    tx.send(t).unwrap();
                    break;
                }
                None => pw_length += 1,
            }
        }
    });
}
