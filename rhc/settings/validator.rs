use std::collections::HashSet;

use super::*;

use crate::error::argument::{
    DETERMINE_ALGORITHM_ERROR, MISSING_TARGET_INPUT_ERROR, MISSING_WORD_LIST_ERROR,
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
    wordlist: &'a Vec<String>,
}

impl<'a> DictionaryValues<'a> {
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
    pub(super) target: Vec<u8>,
    pub(super) thread_count: usize,
    pub(super) algorithm: AlgorithmType,
    pub(super) quiet: bool,
    pub(super) modes: HashSet<Strategy>,
    pub(super) wordlist: Vec<String>,
    pub(super) min_length: usize,
    pub(super) max_length: usize,
    pub(super) charset: CharacterSet,
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

    pub fn quiet(&self) -> bool {
        self.quiet
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

    pub fn charset(&self) -> &CharacterSet {
        &self.charset
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

pub fn validate(
    mut unvalidated: unvalidated::Settings,
) -> Result<ProcessedSettings, ArgumentError> {
    let target = validate_target(unvalidated.target())?;
    if unvalidated.target_type().is_none() {
        let algorithm = determine_algorithm(target)?;
        unvalidated.add_setting(Setting::TargetType(algorithm));
    };
    validate_length(unvalidated.min_length(), unvalidated.max_length())?;
    validate_thread_count(unvalidated.thread_count())?;
    validate_dictionary_mode(&mut unvalidated)?;

    let validated: ProcessedSettings = unvalidated.into();
    Ok(validated)
}

fn validate_target<'a>(target: Option<&'a Vec<u8>>) -> Result<&'a Vec<u8>, ArgumentError> {
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
        20 => {
            println!("No algorithm specified, auto detected to: Sha1");
            Ok(AlgorithmType::Sha1)
        }
        28 => {
            println!("No algorithm specified, auto detected to: Sha224");
            Ok(AlgorithmType::Sha2_224)
        } // , AlgorithmType::Sha2_512_224
        32 => {
            println!("No algorithm specified, auto detected to: Sha256");
            Ok(AlgorithmType::Sha2_256)
        } // ,AlgorithmType::Sha2_512_256
        48 => {
            println!("No algorithm specified, auto detected to: Sha384");
            Ok(AlgorithmType::Sha2_384)
        }
        64 => {
            println!("No algorithm specified, auto detected to: Sha512");
            Ok(AlgorithmType::Sha2_512)
        }
        _ => Err(DETERMINE_ALGORITHM_ERROR),
    }
}

fn validate_dictionary_mode(unvalidated: &mut unvalidated::Settings) -> Result<(), ArgumentError> {
    if unvalidated.modes().contains(&Strategy::Dictionary) {
        unvalidated.wordlist().ok_or(MISSING_WORD_LIST_ERROR)?;
    } else if unvalidated.wordlist().is_some() {
        unvalidated
            .modes()
            .insert(Strategy::Dictionary)
            .then(|| println!("Wordlist detected, enabling Dictionary mode."));
    }
    Ok(())
}

fn validate_length(min_length: usize, max_length: usize) -> Result<(), ArgumentError> {
    if min_length > max_length {
        return Err(ArgumentError::bad_length(min_length, max_length));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> unvalidated::Settings {
        let mut unvalidated = unvalidated::Settings::new();
        unvalidated.add_setting(Setting::Target(vec![
            144, 163, 237, 158, 50, 178, 170, 244, 198, 28, 65, 14, 185, 37, 66, 97, 25, 225, 169,
            220, 83, 212, 40, 106, 222, 153, 168, 9,
        ]));

        unvalidated
    }

    #[test]
    fn test_validate_missing_target() {
        let unvalidated = unvalidated::Settings::new();
        let result = validate(unvalidated);
        assert_eq!(result, Err(MISSING_TARGET_INPUT_ERROR));
    }

    #[test]
    fn test_validate_bad_algorithm() {
        let mut unvalidated = unvalidated::Settings::new();
        unvalidated.add_setting(Setting::Target(vec![123]));
        let result = validate(unvalidated);
        assert_eq!(result, Err(DETERMINE_ALGORITHM_ERROR));
    }

    #[test]
    fn test_validate_bad_thread_count() {
        let thread_target = 999999;
        let mut unvalidated = setup();
        unvalidated.add_setting(Setting::ThreadCount(thread_target));
        let result = validate(unvalidated);
        assert_eq!(
            result,
            Err(ArgumentError::invalid_thread_count(thread_target))
        );
    }

    #[test]
    fn test_validate_bad_length() {
        let min = 5;
        let max = 4;
        let mut unvalidated = setup();
        unvalidated.add_setting(Setting::MaxLength(max));
        unvalidated.add_setting(Setting::MinLength(min));
        let result = validate(unvalidated);
        assert_eq!(result, Err(ArgumentError::bad_length(min, max)));
    }

    #[test]
    fn test_validate_missing_wordlist() {
        let mut unvalidated = setup();
        unvalidated.add_setting(Setting::Mode(Strategy::Dictionary));
        let result = validate(unvalidated);
        assert_eq!(result, Err(MISSING_WORD_LIST_ERROR));
    }

    #[test]
    fn test_validate() -> Result<(), ArgumentError> {
        let mut unvalidated = unvalidated::Settings::new();
        unvalidated.add_setting(Setting::Target(vec![
            144, 163, 237, 158, 50, 178, 170, 244, 198, 28, 65, 14, 185, 37, 66, 97, 25, 225, 169,
            220, 83, 212, 40, 106, 222, 153, 168, 9,
        ]));
        let result = validate(unvalidated)?;
        Ok(())
    }
}
