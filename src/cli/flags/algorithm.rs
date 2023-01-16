use crate::cli::{error::argument::ArgumentError, settings::Setting};

use super::{FlagHelp, FlagInfo, FlagInput};

// const SHORTHAND: char = 'a'; ?
const NAME: &str = "algorithm";
const SHORT_HELP: &str = "The algorithm to target";
const LONG_HELP: &str = "The input should be a valid hash algorithm.
Available algorithms are:
The Sha2 family:
    sha224, sha256, sha384, sha512, sha512_224, sha512_256
Example: rhc [OPTIONS]... --algorithm sha224
";

pub(super) struct Algorithm;

impl FlagInfo for Algorithm {
    fn describe(&self) -> String {
        format!("    --{NAME} \t{SHORT_HELP}")
    }
}

impl FlagHelp for Algorithm {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagInput for Algorithm {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError> {
        Ok(Setting::HashType(value.to_owned()))
    }
}
