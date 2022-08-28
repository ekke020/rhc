use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use crate::core::prelude::*;
use crate::systems::printer::print;

pub fn run_threads(pw_info: PasswordInfo) {
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
