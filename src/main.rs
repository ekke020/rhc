#![allow(warnings)]
mod core;
mod sha2;
mod systems;
mod vulkan;
use std::convert::TryInto;
use std::env;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
const ASCII_TABLE: [char; 95] = [
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2',
    '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
    'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~',
];
fn main() {
    let mut tes = sha2::Sha224::new("test");
    tes.run();
    println!("{}", tes.extract_as_lower_hex());
    // // vulkan::test("test".as_bytes());
    // const size: usize = 1_usize.pow(95);
    // let data:[[u8; 3]; size];
    // add_length_specific::<3>(0, &String::new());
}

pub fn run_threads() {
    // let (tx, rx) = mpsc::channel();
    for i in 0..95 {
        let start_char = (32 + i as u8) as char;
        // spin_up_thread(tx.clone(), start_char, pointer.clone());
    }
    // let received = rx.recv().unwrap();
    // received.print();
}

fn spin_up_thread(
    // tx: Sender<cracker::Cracked>,
    star_char: char,
) {
    thread::spawn(move || {
        loop {
            add_length_specific::<4>(1, &star_char.to_string());
        }
    });
}

fn print_all_up_to(length: usize, prefix: &String) {
    if length == 0 {
        println!("{}", prefix);
        return
    }
    for char in ASCII_TABLE {
        let mut temp = prefix.clone();
        temp.push(char);
        print_all_up_to(length - 1, &temp);
    }
}

fn add_length_specific<const N: usize>(length: usize, prefix: &String) {
    if length == N {
        println!("{:?}", prefix.as_bytes());
        return
    }
    for char in ASCII_TABLE {
        let mut temp = prefix.clone();
        temp.push(char);
        add_length_specific::<N>(length + 1, &temp);
    }
}