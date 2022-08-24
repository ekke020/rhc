mod systems;
mod config;
mod input;
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
    pub use crate::config::Config;
    pub const CRACK_TEST: &str = "99fb2f48c6af4761f904fc85f95eb56190e5d40b1f44ec3a9c1fa319";
    lazy_static! {
        pub static ref MATCHER_INFO: matcher::MatcherInfo = matcher::MatcherInfo::new(CRACK_TEST);
        pub static ref START_TIME: std::time::SystemTime = std::time::SystemTime::now();
    }
}
use prelude::*;
use std::sync::mpsc::Sender;
use std::thread;

fn main() {
    let args = input::create_flag_matcher();
    let config = Config::from(args);
    config.scan();

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
