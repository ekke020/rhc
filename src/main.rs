mod config;
mod input;
mod systems;
#[macro_use]
extern crate lazy_static;
pub mod prelude {
    pub use crate::config::Config;
    pub use crate::systems::cracker;
    pub use crate::systems::filereader;
    pub use crate::systems::hasher::HashType;
    pub use crate::systems::macros;
    pub use crate::systems::password_info::*;
    pub use crate::systems::password_matcher as matcher;
    pub use crate::systems::printer::print;
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
    let pw_info = PasswordInfoBuilder::new(config.get_hash())
        .set_algorithm_type(config.get_algorithm_type())
        .set_password_length(config.get_length())
        .build();

    // let (tx, rx) = mpsc::channel();
    // for i in 0..95 {
    //     let start_char = (32 + i as u8) as char;
    //     spin_up_thread(tx.clone(), start_char, i);
    // }
    // let received = rx.recv().unwrap();
    // received.print();
}

fn spin_up_thread(tx: Sender<PasswordInfo>, star_char: char, num: i32) {
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
