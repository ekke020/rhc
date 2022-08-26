use crate::prelude::*;
use std::time::Instant;

#[derive(Clone)]
pub struct PasswordInfo {
    possible_algorithms: Vec<HashType>,
    hash_type_result: HashType,
    password_length: usize,
    password: Option<String>,
    start_time: Instant,
}

impl Default for PasswordInfo {
    fn default() -> Self {
        Self {
            possible_algorithms: Vec::new(),
            hash_type_result: HashType::default(),
            password_length: 0,
            password: None,
            start_time: Instant::now(),
        }
    }
}

impl PasswordInfo {

    pub fn get_algorithms(&self) -> &Vec<HashType> {
         &self.possible_algorithms
    }

    pub fn get_mut_algorithms(&mut self) -> &mut Vec<HashType> {
        &mut self.possible_algorithms
    }

    pub fn set_password(&mut self, password: String) {
        self.password = Some(password);
    }

    pub fn get_password(&self) -> &String {
        &self
            .password
            .as_ref()
            .expect("Password should never be returned as a None value")
    }

    pub fn set_password_length(&mut self, length: usize) {
        self.password_length = length;
    }

    pub fn get_password_length(&self) -> &usize {
        &self.password_length
    }

    pub fn set_hash_type_result(&mut self, hash_type: HashType) {
        self.hash_type_result = hash_type;
    }

    pub fn get_hash_type_result(&self) -> &HashType {
        &self.hash_type_result
    }

    pub fn get_elapsed_time(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}

pub struct PasswordInfoBuilder<'a> {
    pw_info: PasswordInfo,
    hash: &'a str,
}

impl<'a> PasswordInfoBuilder<'a> {
    pub fn new(hash: &'a str) -> Self {
        Self {
            pw_info: PasswordInfo::default(),
            hash,
        }
    }

    pub fn set_algorithm_type(&mut self, algorithm_type: &Option<String>) -> &mut Self {
        if let Some(algorithm) = algorithm_type {
            let verified = hash::get_algorithm(algorithm, self.hash.to_string());
            self.pw_info.get_mut_algorithms().push(verified);
        } else {
            hash::set_possible_algorithms(self);
        }
        self
    }

    pub fn set_password_length(&mut self, length: Option<usize>) -> &mut Self {
        if let Some(length) = length {
            self.pw_info.set_password_length(length);
        }
        self
    }

    pub fn build(&self) -> PasswordInfo {
        let pw_info = self.pw_info.clone();
        drop(self);
        pw_info
    }
}

mod hash {
    use crate::prelude::*;

    pub fn get_algorithm(algorithm: &String, hash: String) -> HashType {
        match algorithm.to_uppercase().as_str() {
            "SHA_224" => HashType::Sha224(hash),
            "SHA_256" => HashType::Sha256(hash),
            "SHA_384" => HashType::Sha384(hash),
            "SHA_512" => HashType::Sha512(hash),
            "SHA_512_224" => HashType::Sha512_224(hash),
            "SHA_512_256" => HashType::Sha512_256(hash),
            _ => {
                println!("Failed to evaluate algorithm: {}, (-a, --all for a list of available algorithms)", algorithm);
                std::process::exit(1);
            }
        }
    }

    pub fn set_possible_algorithms<'a>(pib: &mut PasswordInfoBuilder) {
        let algorithms = pib.pw_info.get_mut_algorithms();
        match pib.hash.as_bytes().len() * 4 {
            224 => {
                algorithms.push(HashType::Sha224(String::from(pib.hash)));
                algorithms.push(HashType::Sha512_224(String::from(pib.hash)));
            }
            256 => {
                algorithms.push(HashType::Sha256(String::from(pib.hash)));
                algorithms.push(HashType::Sha512_256(String::from(pib.hash)));
            }
            384 => algorithms.push(HashType::Sha384(String::from(pib.hash))),
            512 => algorithms.push(HashType::Sha512(String::from(pib.hash))),
            _ => {
                print::failed_to_establish_algorithm(pib.hash);
                std::process::exit(1);
            }
        }
    }
}
