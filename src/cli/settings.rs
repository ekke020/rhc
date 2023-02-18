use crate::{algorithm::AlgorithmType, core::crack::mode::Mode};

use super::error::argument::ArgumentError;
use std::collections::HashSet;
pub enum Setting {
    HashInput(Vec<u8>), // Should be renamed to target?
    HashType(AlgorithmType),
    HashLength(u32), // Should be renamed to start length? 
    Verbose(bool),
    Wordlist(Vec<String>),
    Mode(Mode),
}
// TODO: Might want to rename the variables in the Struct.
// TODO: Look over missing memebers of the Struct.
// TODO: Consider changing hash_input to be a list, this would require a lot of work...
#[derive(Debug, Clone)]
pub struct InputOptions {
    hash_input: Option<Vec<u8>>, // Should be renamed to target
    hash_type: Option<AlgorithmType>, // Should be renamed to target_type
    hash_length: Option<u32>,  // Should be renamed to start_length?
    wordlist: Option<Vec<String>>,
    verbose: bool,
    modes: HashSet<Mode>,
    thread_count: usize,
}

impl InputOptions {
    pub(super) fn new() -> Self {
        InputOptions {
            hash_input: None,
            hash_type: None,
            hash_length: None,
            wordlist: None,
            verbose: false,
            modes: HashSet::from([Mode::Incremental]),
            thread_count: num_cpus::get(),
        }
    }
    pub fn add_setting(&mut self, setting: Setting) {
        match setting {
            Setting::HashInput(value) => self.hash_input = Some(value),
            Setting::HashType(value) => self.hash_type = Some(value),
            Setting::HashLength(value) => self.hash_length = Some(value),
            Setting::Verbose(value) => self.verbose = value,
            Setting::Wordlist(value) => self.wordlist = Some(value),
            Setting::Mode(mode) => {
                self.modes.insert(mode);
            }
        }
    }

    pub fn get_hash_input(&mut self) -> Option<Vec<u8>> {
        self.hash_input.take()
    }

    pub fn get_hash_type(&mut self) -> Option<AlgorithmType> {
        self.hash_type.take()
    }

    pub fn get_hash_length(&mut self) -> Option<u32> {
        self.hash_length.take()
    }

    pub fn get_wordlist(&mut self) -> Option<Vec<String>> {
        self.wordlist.take()
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn is_mode(&self, mode: Mode) -> bool {
        self.modes.contains(&mode)
    }
}

mod validator {
    use std::collections::HashSet;

    use super::InputOptions;
    use crate::{cli::error::argument::{ArgumentError, MISSING_TARGET_INPUT_ERROR, DETERMINE_ALGORITHM_ERROR, MISSING_WORD_LIST_ERROR}, algorithm::AlgorithmType, core::crack::mode::Mode};

    pub struct Settings {
        target: Vec<u8>,
        thread_count: usize,
        algorithm: AlgorithmType,
        is_verbose: bool,
        modes: HashSet<Mode>,
        wordlist: Vec<String>,
    }

    pub fn validate(settings: &mut InputOptions) -> Result<(), ArgumentError> {
        Ok(())
    }

    fn validate_target(target: Option<Vec<u8>>) -> Result<Vec<u8>, ArgumentError> {
        let target = target.ok_or_else(|| MISSING_TARGET_INPUT_ERROR)?;
        Ok(target)
    }

    fn validate_thread_count(thread_count: usize) -> Result<usize, ArgumentError> {
        let available_count = num_cpus::get();
        match thread_count > available_count {
            true => Err(ArgumentError::invalid_thread_count(thread_count)),
            false => Ok(thread_count),
        }
    }

    fn determine_algorithm(target: &Vec<u8>) -> Result<AlgorithmType, ArgumentError> {
        match target.len() {
            28 => Ok(AlgorithmType::Sha2_224), // , AlgorithmType::Sha2_512_224
            32 => Ok(AlgorithmType::Sha2_256), // ,AlgorithmType::Sha2_512_256
            48 => Ok(AlgorithmType::Sha2_384),
            64 => Ok(AlgorithmType::Sha2_512),
            _ => Err(DETERMINE_ALGORITHM_ERROR),
        }
    }

    fn validate_wordlist(wordlist: Option<Vec<String>>, modes: &mut HashSet<Mode>) -> Result<Vec<String>, ArgumentError> {
        match wordlist {
            Some(list) => {
                modes.insert(Mode::Dictionary).then(|| println!("Wordlist detected, enabling Dictionary mode."));
                Ok(list)
            },
            None => {
                if modes.contains(&Mode::Dictionary) {
                    return Err(MISSING_WORD_LIST_ERROR)
                }
                Ok(vec![])
            },
        }
    }
}
