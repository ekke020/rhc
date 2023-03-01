use std::vec;

use crate::{
    cli::settings::{self, UnvalidatedSettings},
    sha2::Sha224, algorithm::{AlgorithmType, Algorithm},
};

use super::error::core::{CoreError, INVALID_ALGORITHM_ERROR, MISSING_HASH_INPUT_ERROR, MALFORMED_HASH_ERROR};

// TODO: This Struct is considered depricated. Work on decompiling and restructure a thread-safe alternative
#[derive(Debug, Clone)]
pub struct Package {
    target_length: usize,
    target: Vec<u8>,
    algorithm: AlgorithmType,
    is_verbose: bool,
    thread_count: usize,
    // wordlist: Option<Vec<String>>
}

impl Package {
    pub fn assemble(settings: &mut UnvalidatedSettings) -> Result<Self, CoreError> {
        println!("{}", num_cpus::get());
        let target = set_target(settings.get_target())?;
        let algorithm = match settings.get_target_type() {
            Some(algorithm) => algorithm,
            None => determine_algorithm(&target)?,
        };
        Ok(Self {
            target_length: 2,
            target,
            algorithm,
            is_verbose: settings.is_verbose(),
            thread_count: num_cpus::get(),
            // wordlist: None,
        })
    }

    // pub fn set_wordlist(&mut self, wordlist: Vec<String>) {
    //     self.wordlist = Some(wordlist);
    // }
    
    pub fn get_target(&self) -> &Vec<u8> {
        &self.target
    }

    pub fn get_algorithm(&self) -> Box<dyn Algorithm> {
        self.algorithm.generate_algorithm()
    }

    pub fn get_thread_count(&self) -> usize {
        self.thread_count
    }
}
// TODO: MISSING_HASH_INPUT_ERROR should remain when validating the settings
fn set_target(target: Option<Vec<u8>>) -> Result<Vec<u8>, CoreError> {
    let bytes = target.ok_or_else(|| MISSING_HASH_INPUT_ERROR)?;
    Ok(bytes)
}

fn determine_algorithm(value: &Vec<u8>) -> Result<AlgorithmType, CoreError> {
    // TODO: Come up with a better solution than this
    match value.len() {
        28 => Ok(AlgorithmType::Sha2_224), // , AlgorithmType::Sha2_512_224
        32 => Ok(AlgorithmType::Sha2_256), // ,AlgorithmType::Sha2_512_256 
        48 => Ok(AlgorithmType::Sha2_384), 
        64 => Ok(AlgorithmType::Sha2_512), 
        _ => Err(CoreError::determine_algorithm(value)),
    }
}
