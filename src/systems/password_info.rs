use crate::prelude::*;
use std::time::Instant;
#[derive(Clone)]
pub struct PasswordInfo {
    possible_hash_types: Vec<hasher::HashType>,
    hash_type_result: Option<hasher::HashType>,
    password: Option<String>,
    start_time: Instant,
}

impl PasswordInfo {
    pub fn new(possible_hash_types: Vec<hasher::HashType>) -> Self {
        Self {
            possible_hash_types,
            hash_type_result: None,
            password: None,
            start_time: Instant::now(),
        }
    }
    pub fn print(&self) {
        println!("Hash type: {:#?}\nPassword: {:#?}", self.hash_type_result, self.password);
        let duration = self.start_time.elapsed();
        print::elapsed_time(duration.as_secs());
    }
}
