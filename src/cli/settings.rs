use crate::{algorithm::AlgorithmType, core::crack::mode::Mode};

use super::error::argument::ArgumentError;
use std::collections::HashSet;
pub enum Setting {
    Target(Vec<u8>),
    TargetType(AlgorithmType),
    MinLength(u32),
    MaxLength(u32),
    Verbose(bool),
    Wordlist(Vec<String>),
    Mode(Mode),
}

#[derive(Debug, Clone)]
pub struct UnvalidatedSettings {
    target: Option<Vec<u8>>,
    target_type: Option<AlgorithmType>,
    min_length: u32,
    max_length: u32,
    wordlist: Option<Vec<String>>,
    verbose: bool,
    modes: HashSet<Mode>,
    thread_count: usize,
}

impl UnvalidatedSettings {
    pub(super) fn new() -> Self {
        UnvalidatedSettings {
            target: None,
            target_type: None,
            min_length: 1,
            max_length: 999,
            wordlist: None,
            verbose: false,
            modes: HashSet::from([Mode::Incremental]),
            thread_count: num_cpus::get(),
        }
    }
    pub fn add_setting(&mut self, setting: Setting) {
        match setting {
            Setting::Target(value) => self.target = Some(value),
            Setting::TargetType(value) => self.target_type = Some(value),
            Setting::MinLength(value) => self.min_length = value,
            Setting::MaxLength(value) => self.max_length = value,
            Setting::Verbose(value) => self.verbose = value,
            Setting::Wordlist(value) => self.wordlist = Some(value),
            Setting::Mode(mode) => {
                self.modes.insert(mode);
            }
        }
    }

    pub fn get_target(&mut self) -> Option<Vec<u8>> {
        self.target.take()
    }

    pub fn get_target_type(&mut self) -> Option<AlgorithmType> {
        self.target_type.take()
    }

    pub fn get_min_length(&mut self) -> u32 {
        self.min_length
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

pub (super) mod validator {
    use std::collections::HashSet;

    use super::UnvalidatedSettings;
    use crate::{
        algorithm::AlgorithmType,
        cli::error::argument::{
            ArgumentError, DETERMINE_ALGORITHM_ERROR, MISSING_TARGET_INPUT_ERROR,
            MISSING_WORD_LIST_ERROR,
        },
        core::crack::Mode,
    };
    // TODO: This should hold only shared values between modes.
    // TODO: Look at chatGPT for clarification.
    pub struct ProcessedSettings {
        target: Vec<u8>,
        thread_count: usize,
        algorithm: AlgorithmType,
        verbose: bool,
        modes: HashSet<Mode>,
        wordlist: Vec<String>,
        min_length: u32,
        max_length: u32,
    }

    impl ProcessedSettings {
        pub fn test(&self) {

        }
    }

    pub fn validate(raw_settings: UnvalidatedSettings) -> Result<ProcessedSettings, ArgumentError> {
        let target = validate_target(raw_settings.target)?;
        let algorithm = match raw_settings.target_type {
            Some(algorithm) => algorithm,
            None => determine_algorithm(&target)?,
        };
        validate_length(raw_settings.min_length, raw_settings.max_length)?;
        let mut modes = raw_settings.modes;
        let wordlist = validate_wordlist(raw_settings.wordlist, &mut modes)?;
        let settings = ProcessedSettings {
            target,
            thread_count: validate_thread_count(raw_settings.thread_count)?,
            algorithm,
            verbose: raw_settings.verbose,
            modes,
            wordlist,
            min_length: raw_settings.min_length,
            max_length: raw_settings.max_length,
        };
        Ok(settings)
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

    fn validate_wordlist(
        wordlist: Option<Vec<String>>,
        modes: &mut HashSet<Mode>,
    ) -> Result<Vec<String>, ArgumentError> {
        match wordlist {
            Some(list) => {
                modes
                    .insert(Mode::Dictionary)
                    .then(|| println!("Wordlist detected, enabling Dictionary mode."));
                Ok(list)
            }
            None => {
                if modes.contains(&Mode::Dictionary) {
                    return Err(MISSING_WORD_LIST_ERROR);
                }
                Ok(vec![])
            }
        }
    }

    fn validate_length(min_length: u32, max_length: u32) -> Result<(), ArgumentError> {
        if min_length > max_length {
            return Err(ArgumentError::bad_length(min_length, max_length));
        }
        Ok(())
    }
}
