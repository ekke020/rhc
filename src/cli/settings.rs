use super::error::argument::ArgumentError;
use crate::{algorithm::AlgorithmType, core::charset::CharacterSet};
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Strategy {
    Dictionary,
    Incremental,
}

impl Strategy {
    pub fn from(value: &str) -> Option<Self> {
        match value {
            "dictionary" => Some(Self::Dictionary),
            "incremental" => Some(Self::Incremental),
            _ => None,
        }
    }
}

pub enum Setting {
    Target(Vec<u8>),
    TargetType(AlgorithmType),
    MinLength(usize),
    MaxLength(usize),
    Verbose(bool),
    Wordlist(Vec<String>),
    ThreadCount(usize),
    Mode(Strategy),
    Charset(CharacterSet),
}

#[derive(Debug, Clone)]
pub struct UnvalidatedSettings {
    target: Option<Vec<u8>>,
    target_type: Option<AlgorithmType>,
    min_length: usize,
    max_length: usize,
    wordlist: Option<Vec<String>>,
    verbose: bool,
    modes: HashSet<Strategy>,
    charset: CharacterSet,
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
            modes: HashSet::from([Strategy::Incremental]),
            charset: CharacterSet::Common,
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
            Setting::ThreadCount(count) => self.thread_count = count,
            Setting::Charset(set) => self.charset = set,
            Setting::Mode(mode) => { self.modes.insert(mode); }
        }
    }
}

pub(super) mod validator {
    use std::collections::HashSet;

    use super::{Strategy, UnvalidatedSettings};
    use crate::{
        algorithm::AlgorithmType,
        cli::error::argument::{
            ArgumentError, DETERMINE_ALGORITHM_ERROR, MISSING_TARGET_INPUT_ERROR,
            MISSING_WORD_LIST_ERROR,
        }, core::charset::CharacterSet,
    };

    pub struct IncrementalValues {
        thread_count: usize,
        min_length: usize,
        max_length: usize,
        charset: &'static [u8],
    }

    impl IncrementalValues {
        pub fn thread_count(&self) -> usize {
            self.thread_count
        }
    
        pub fn max_length(&self) -> usize {
            self.max_length
        }

        pub fn min_length(&self) -> usize {
            self.min_length
        }

        pub fn charset(&self) -> &'static [u8] {
            self.charset
        }
    }

    pub struct DictionaryValues<'a> {
        thread_count: usize,
        wordlist: &'a Vec<String>
    }

    impl <'a>DictionaryValues<'a> {
        pub fn thread_count(&self) -> usize {
            self.thread_count
        }
    
        pub fn wordlist(&self) -> &'a Vec<String> {
            self.wordlist
        }
    }
    // TODO: Consider changing modes to be specific structs instead...
    #[derive(Debug, PartialEq)]
    pub struct ProcessedSettings {
        target: Vec<u8>,
        thread_count: usize,
        algorithm: AlgorithmType,
        verbose: bool,
        modes: HashSet<Strategy>,
        wordlist: Vec<String>,
        min_length: usize,
        max_length: usize,
        charset: CharacterSet,
    }

    impl ProcessedSettings {
        pub fn target(&self) -> &Vec<u8> {
            &self.target
        }
    
        pub fn thread_count(&self) -> usize {
            self.thread_count
        }

        pub fn algorithm(&self) -> &AlgorithmType {
            &self.algorithm
        }
    
        pub fn verbose(&self) -> bool {
            self.verbose
        }

        pub fn modes(&self) -> &HashSet<Strategy> {
            &self.modes
        }

        pub fn wordlist(&self) -> &[String] {
            self.wordlist.as_ref()
        }

        pub fn min_length(&self) -> usize {
            self.min_length
        }

        pub fn max_length(&self) -> usize {
            self.max_length
        }

        pub fn incremental_values(&self) -> IncrementalValues {
            IncrementalValues {
                thread_count: self.thread_count,
                min_length: self.min_length,
                max_length: self.max_length,
                charset: self.charset.get_table(),
            }
        }

        pub fn dictionary_values(&self) -> Option<DictionaryValues> {
            self.modes
                .get(&Strategy::Dictionary)
                .and(Some(DictionaryValues {
                    thread_count: self.thread_count,
                    wordlist: &self.wordlist,
                }))
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
        let wordlist = validate_dictionary_mode(raw_settings.wordlist, &mut modes)?;
        let settings = ProcessedSettings {
            target,
            thread_count: validate_thread_count(raw_settings.thread_count)?,
            algorithm,
            verbose: raw_settings.verbose,
            modes,
            wordlist,
            min_length: raw_settings.min_length,
            max_length: raw_settings.max_length,
            charset: raw_settings.charset,
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
            28 => {
                println!("No algorithm specified, auto detected to: Sha224");
                Ok(AlgorithmType::Sha2_224)
            }, // , AlgorithmType::Sha2_512_224
            32 => {
                println!("No algorithm specified, auto detected to: Sha256");
                Ok(AlgorithmType::Sha2_256)
            }, // ,AlgorithmType::Sha2_512_256
            48 => {
                println!("No algorithm specified, auto detected to: Sha384");
                Ok(AlgorithmType::Sha2_384)
            },
            64 => {
                println!("No algorithm specified, auto detected to: Sha512");
                Ok(AlgorithmType::Sha2_512)
            },
            _ => Err(DETERMINE_ALGORITHM_ERROR),
        }
    }

    fn validate_dictionary_mode(
        wordlist: Option<Vec<String>>,
        modes: &mut HashSet<Strategy>,
    ) -> Result<Vec<String>, ArgumentError> {
        if modes.contains(&Strategy::Dictionary) {
            let list = wordlist.ok_or(MISSING_WORD_LIST_ERROR)?;
            return Ok(list);
        }
        match wordlist {
            Some(list) => {
                modes
                    .insert(Strategy::Dictionary)
                    .then(|| println!("Wordlist detected, enabling Dictionary mode."));
                Ok(list)
            }
            None => Ok(vec![]),
        }
    }

    fn validate_length(min_length: usize, max_length: usize) -> Result<(), ArgumentError> {
        if min_length > max_length {
            return Err(ArgumentError::bad_length(min_length, max_length));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::error::argument::{MISSING_TARGET_INPUT_ERROR, DETERMINE_ALGORITHM_ERROR, MISSING_WORD_LIST_ERROR};


    fn setup() -> UnvalidatedSettings {
        let mut unvalidated = UnvalidatedSettings::new();
        unvalidated.add_setting(Setting::Target(vec![
            144, 163, 237, 158, 50, 178, 170, 244, 198, 
            28, 65, 14, 185, 37, 66, 97, 25, 225, 169,
            220, 83, 212, 40, 106, 222, 153, 168, 9,
        ]));

        unvalidated
    }
    #[test]
    fn test_validate_missing_target() {
        let unvalidated = UnvalidatedSettings::new();
        let result = validator::validate(unvalidated);
        assert_eq!(result, Err(MISSING_TARGET_INPUT_ERROR));
    }

    #[test]
    fn test_validate_bad_algorithm() {
        let mut unvalidated = UnvalidatedSettings::new();
        unvalidated.add_setting(Setting::Target(vec![123]));
        let result = validator::validate(unvalidated);
        assert_eq!(result, Err(DETERMINE_ALGORITHM_ERROR));
    }


    #[test]
    fn test_validate_bad_thread_count() {
        let thread_target = 999999;
        let mut unvalidated = setup();
        unvalidated.add_setting(Setting::ThreadCount(thread_target));
        let result = validator::validate(unvalidated);
        assert_eq!(result, Err(ArgumentError::invalid_thread_count(thread_target)));
    }
    
    #[test]
    fn test_validate_bad_length() {
        let min = 5;
        let max = 4;
        let mut unvalidated = setup();
        unvalidated.add_setting(Setting::MaxLength(max));
        unvalidated.add_setting(Setting::MinLength(min));
        let result = validator::validate(unvalidated);
        assert_eq!(result, Err(ArgumentError::bad_length(min, max)));
    }

    #[test]
    fn test_validate_missing_wordlist() {
        let mut unvalidated = setup();
        unvalidated.add_setting(Setting::Mode(Strategy::Dictionary));
        let result = validator::validate(unvalidated);
        assert_eq!(result, Err(MISSING_WORD_LIST_ERROR));
    }

    #[test]
    fn test_validate() -> Result<(), ArgumentError> {
        let mut unvalidated = UnvalidatedSettings::new();
        unvalidated.add_setting(Setting::Target(vec![
            144, 163, 237, 158, 50, 178, 170, 244, 198, 
            28, 65, 14, 185, 37, 66, 97, 25, 225, 169,
            220, 83, 212, 40, 106, 222, 153, 168, 9,
        ]));
        let result = validator::validate(unvalidated)?;
        Ok(())
    }

}