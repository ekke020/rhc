use std::vec;

use crate::{
    cli::settings::{self, GlobalSettings},
    sha2::Sha224, algorithm::AlgorithmType,
};

use super::error::core::{CoreError, INVALID_ALGORITHM_ERROR, MISSING_HASH_INPUT_ERROR, MALFORMED_HASH_ERROR};

#[derive(Debug, Clone)]
pub struct Package {
    target_length: usize,
    target: Vec<u8>,
    algorithms: Vec<AlgorithmType>,
    is_verbose: bool,
}

impl Package {
    pub fn assemble(settings: &mut GlobalSettings) -> Result<Self, CoreError> {
        let target = set_target(settings.get_hash_input())?;
        let algorithms = match settings.get_hash_type() {
            Some(algorithm) => vec![algorithm],
            None => determine_algorithm(&target)?,
        };
        Ok(Self {
            target_length: settings.get_hash_length().unwrap_or(1) as usize,
            target,
            algorithms,
            is_verbose: settings.is_verbose(),
        })
    }

    pub fn get_target(&self) -> &Vec<u8> {
        &self.target
    }

    pub fn get_algorithms(&self) -> &Vec<AlgorithmType> {
        &self.algorithms
    }
}

fn set_target(target: Option<String>) -> Result<Vec<u8>, CoreError> {
    let value = target.ok_or_else(|| MISSING_HASH_INPUT_ERROR)?;
    let bytes = hex_string_to_bytes(&value)?;
    Ok(bytes)
}

fn hex_string_to_bytes(hex_string: &str) -> Result<Vec<u8>, CoreError> {
    let hex_values = hex_string.as_bytes();
    let mut result = Vec::new();
    if hex_values.len() % 2 != 0 {
        return Err(MALFORMED_HASH_ERROR)
    }
    for i in (0..hex_values.len()).step_by(2) {
        let hex_value = &hex_values[i..i + 2];
        let value = u8::from_str_radix(std::str::from_utf8(hex_value).unwrap(), 16)?;
        result.push(value);
    }
    Ok(result)
}

fn determine_algorithm(value: &Vec<u8>) -> Result<Vec<AlgorithmType>, CoreError> {
    match value.len() {
        28 => Ok(vec![AlgorithmType::Sha2_224, AlgorithmType::Sha2_512_224]), 
        32 => Ok(vec![AlgorithmType::Sha2_256, AlgorithmType::Sha2_512_256]), 
        48 => Ok(vec![AlgorithmType::Sha2_384]), 
        64 => Ok(vec![AlgorithmType::Sha2_512]), 
        _ => Err(CoreError::determine_algorithm(value)),
    }
}
