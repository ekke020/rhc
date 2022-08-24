use crate::prelude::*;
use std::time::Instant;
#[derive(Clone)]
pub struct PasswordInfo {
    // possible_hash_types: Vec<hasher::HashType>,
    first_hash_type: HashType,
    second_hash_type: HashType,
    hash_type_result: HashType,
    password_length: usize,
    password: Option<String>,
    start_time: Instant,
}

impl Default for PasswordInfo {
    fn default() -> Self {
        Self {
            first_hash_type: HashType::default(),
            second_hash_type: HashType::default(),
            hash_type_result: HashType::default(),
            password_length: 0,
            password: None,
            start_time: Instant::now(),
        }
    }
}

impl PasswordInfo {
    pub fn set_password(&mut self, password: String) {
        self.password = Some(password);
    }

    pub fn get_password(&self) -> &String {
        &self
            .password.as_ref()
            .expect("Password should never be returned as a None value")
    }

    pub fn set_password_length(&mut self, length: usize) { 
        self.password_length = length;
    }

    pub fn get_password_length(&self) -> &usize {
        &self.password_length
    }

    pub fn set_first_hash_type(&mut self, hash_type: HashType) {
        self.first_hash_type = hash_type;
    }

    pub fn get_first_hash_type(&self) -> &HashType {
        &self.first_hash_type
    }

    pub fn set_second_hash_type(&mut self, hash_type: HashType) {
        self.second_hash_type = hash_type;
    }

    pub fn get_second_hash_type(&self) -> &HashType {
        &self.second_hash_type
    }

    pub fn set_hash_type_result(&mut self, hash_type: HashType) {
        self.hash_type_result = hash_type;
    }

    pub fn get_hash_type_result(&self) -> &HashType {
        &self.hash_type_result
    }

    pub fn print(&self) {
        println!(
            "Hash type: {:#?}\nPassword: {:#?}",
            self.hash_type_result, self.password
        );
        let duration = self.start_time.elapsed();
        print::elapsed_time(duration.as_secs());
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
            self.pw_info.set_first_hash_type(hash::get_algorithm(algorithm, self.hash.to_string()));
        } else {
            match hash::get_possible_algorithm(self.hash) {
                Ok(hash_type) => {
                    self.pw_info.set_first_hash_type(hash_type.0);
                    self.pw_info.set_second_hash_type(hash_type.1);
                },
                Err(e) => e.print_and_exit(),
            }
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
    #[derive(Debug)]
    pub struct HasherError<'a>(&'a str);

    impl<'a> HasherError<'a> {
        pub fn print_and_exit(self) {
            println!("{}", self.0);
            std::process::exit(1);
        }
    }

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
    // TODO: Refactor this function so it dosn't use tuples.
    pub fn get_possible_algorithm<'a>(
        input: &str,
    ) -> Result<(HashType, HashType), HasherError<'a>> {
        let hash;
        match input.as_bytes().len() * 4 {
            224 => {
                hash = (
                    HashType::Sha224(input.to_owned()),
                    HashType::Sha512_224(input.to_owned()),
                )
            }
            256 => {
                hash = (
                    HashType::Sha256(input.to_owned()),
                    HashType::Sha512_256(input.to_owned()),
                )
            }
            384 => hash = (HashType::Sha384(input.to_owned()), HashType::Empty),
            512 => hash = (HashType::Sha512(input.to_owned()), HashType::Empty),
            _ => return Err(HasherError("Unable to detect algorithm...")),
        }
        Ok(hash)
    }
}
