use std::vec;

use crate::{
    cli::settings::{self, AlgorithmType, GlobalSettings},
    sha2::Sha224,
};

use super::error::core::{CoreError, INVALID_ALGORITHM_ERROR, MISSING_HASH_INPUT_ERROR};

#[derive(Debug, Clone)]
pub struct Package {
    target_length: usize,
    target: Vec<u8>,
    algorithm: Vec<AlgorithmType>,
    is_verbose: bool,
}

impl Package {
    pub fn assemble(mut settings: GlobalSettings) -> Result<Self, CoreError> {
        let target = set_target(settings.get_hash_input())?;
        let algorithm = match settings.get_hash_type() {
            Some(algorithm) => vec![algorithm],
            None => determine_algorithm(&target)?,
        };
        Ok(Self {
            target_length: settings.get_hash_length().unwrap_or(1) as usize,
            target,
            algorithm,
            is_verbose: settings.is_verbose(),
        })
    }
}

fn set_target(target: Option<String>) -> Result<Vec<u8>, CoreError> {
    let value = target.ok_or_else(|| MISSING_HASH_INPUT_ERROR)?;
    Ok(value.as_bytes().to_owned())
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
