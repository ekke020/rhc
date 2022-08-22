use crate::prelude::*;

pub struct MatcherInfo {
    hash_types: Vec<hasher::HashType>,
}

impl MatcherInfo {
    pub fn new(hash: &str) -> Self {
        match get_possible_algorithm(hash) {
            Ok(result) => {
                return MatcherInfo {
                    hash_types: vec![result.0, result.1],
                }
            }
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        }
    }

    pub fn print(&self) {
        println!("Possible hashtypes:");
        self.hash_types
            .iter()
            .filter(|hash| **hash != hasher::HashType::Empty)
            .for_each(|hash| println!("\t{}", hash.to_string()));
    }

    pub fn try_password(&self, password: &str) -> Option<PW::PasswordInfo> {
        let is_equal = self
            .hash_types
            .iter()
            .find(|hash_type| hash_type.is_match(password));
        match is_equal {
            Some(hash_type) => {
                // TODO: Refactor the returned value.
                // let pw_info = PW::PasswordInfo::new(
                //     hash_type.clone(),
                //     password.to_owned(),
                // );
                let pw_info = PW::PasswordInfo::new(vec![]);
                return Some(pw_info);
            }
            None => return None,
        }
    }
}

fn get_possible_algorithm(input: &str) -> Result<(hasher::HashType, hasher::HashType), String> {
    let hash;
    match input.as_bytes().len() * 4 {
        224 => {
            hash = (
                hasher::HashType::Sha224(input.to_owned()),
                hasher::HashType::Sha512_224(input.to_owned()),
            )
        }
        256 => {
            hash = (
                hasher::HashType::Sha256(input.to_owned()),
                hasher::HashType::Sha512_256(input.to_owned()),
            )
        }
        384 => {
            hash = (
                hasher::HashType::Sha384(input.to_owned()),
                hasher::HashType::Empty,
            )
        }
        512 => {
            hash = (
                hasher::HashType::Sha512(input.to_owned()),
                hasher::HashType::Empty,
            )
        }
        _ => return Err("Unable to detect algorithm...".to_string()),
    }
    Ok(hash)
}
