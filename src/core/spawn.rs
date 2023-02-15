use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{char::from_u32, str::from_utf8, thread};

use crate::algorithm::{self, Algorithm};

use super::crack;
use super::crack::consts::{ASCII_95_TABLE, COMMON_TABLE, NO_SPECIAL_TABLE};
use super::crack::{Dictionary, Incremental};
use super::crack::result::PasswordMatch;
use super::error::core::CoreError;
use super::package::Package;

// pub(super) fn resource_job(
//     package: Package,
//     wordlist: Vec<String>,
// ) -> Result<Vec<JoinHandle<Option<PasswordMatch>>>, CoreError> {
//     let num_cores = num_cpus::get();
//     let chunk_size = wordlist.len() / num_cores;
//     let mut threads = vec![];
//     // TODO: This needs to be optimized. Need a way to split the wordlist more evenly.
//     let mut chunks = wordlist
//         .chunks(chunk_size)
//         .map(|chunk| chunk.to_vec())
//         .collect::<Vec<Vec<String>>>();

//     let mut wordlist_rf = Arc::new(Mutex::new(chunks));

//     for i in 0..=package.get_thread_count() {
//         let p = package.clone();
//         let wordlist = wordlist_rf.clone();
//         threads.push(thread::spawn(move || {
//             // Create an instance of resource and run it on the thread.
//             let target = p.get_target();
//             let chunk = wordlist.lock().unwrap().pop().unwrap();
//             let mut resources: Vec<Dictionary> = p
//                 .get_algorithms()
//                 .iter()
//                 .map(|algorithm| algorithm.get_algorithm())
//                 .map(|algorithm| Dictionary::from(target, &chunk, algorithm))
//                 .collect();
//             for mut resource in resources {
//                 if let Some(result) = resource.run() {
//                     return Some(result);
//                 }
//             }
//             None
//         }));
//     }
//     Ok(threads)
// }

pub(super) fn test_incremental(package: Package) -> Receiver<Option<PasswordMatch>>  {
    let (tx, rx) = mpsc::channel();
    for i in 0..package.get_thread_count() {
        let p = package.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            // Create an instance of incremental and run it on the thread.
            let mut runner = Incremental::un_pack(p, i);
            let result = runner.run();
            tx.send(result);
        });
    };
    rx
}

pub(super) fn test_dictionary(package: Package, wordlist: Option<Vec<String>>) {
    // let num_cores = num_cpus::get();
    // let chunk_size = wordlist.len() / num_cores;
    // let mut chunks = wordlist
    //     .chunks(chunk_size)
    //     .map(|chunk| chunk.to_vec())
    //     .collect::<Vec<Vec<String>>>();
}
