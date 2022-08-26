mod config;
mod input;
mod systems;
pub mod prelude {
    pub use crate::config::Config;
    pub use crate::systems::cracker;
    pub use crate::systems::filereader;
    pub use crate::systems::hasher::HashType;
    pub use crate::systems::macros;
    pub use crate::systems::password_info::*;
    pub use crate::systems::printer::print;
    pub const CRACK_TEST: &str = "99fb2f48c6af4761f904fc85f95eb56190e5d40b1f44ec3a9c1fa319";
}
use prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
fn main() {
    let args = input::create_flag_matcher();
    let config = Config::from(args);
    config.scan();
    let pw_info = PasswordInfoBuilder::new(config.get_hash())
        .set_algorithm_type(config.get_algorithm_type())
        .set_password_length(config.get_length())
        .build();
    let pointer = Arc::new(pw_info);
    let (tx, rx) = mpsc::channel();
    for i in 0..95 {
        let start_char = (32 + i as u8) as char;
        spin_up_thread(tx.clone(), start_char, pointer.clone());
    }
    let received = rx.recv().unwrap();
    received.print();
    print::elapsed_time(pointer.get_elapsed_time());
}

fn spin_up_thread(
    tx: Sender<cracker::Cracked>,
    star_char: char,
    pw_info_pointer: Arc<PasswordInfo>,
) {
    thread::spawn(move || {
        let mut pw_length = pw_info_pointer.get_password_length().clone();
        let algorithms = pw_info_pointer.get_algorithms();
        loop {
            let cracked =
                cracker::run_crack(algorithms, pw_length, &char::to_string(&star_char), None);
            match cracked {
                Some(t) => {
                    tx.send(t).unwrap();
                    break;
                }
                None => pw_length += 1,
            };
        }
    });
}
