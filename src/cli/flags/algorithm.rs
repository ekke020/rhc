use crate::cli::{error::argument::ArgumentError, settings::Setting};

use super::{FlagHelp, FlagInfo, FlagInput};

// const SHORTHAND: char = 'a'; ?
const NAME: &str = "algorithm";
const SHORT_HELP: &str = "The algorithm to target";
const LONG_HELP: &str = "Flag: --algorithm
Details:
    input type: string (specific algorithm to target)
Supported algorithms: 
    the Sha2 family:
        sha224, sha256, sha384,
        sha512, sha512_224, sha512_256
Description:
    The --algorithm flag specifies the algorithm to target when cracking a hash.
    The tool will attempt to automatically detect the algorithm used to create the hash, 
    but this may not always be possible. If the algorithm is known, 
    it is recommended to provide it as a flag to avoid the increased runtime and potential 
    false positives that come with trying multiple algorithms.
Example: 
    rhc [OPTIONS]... --algorithm sha224
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
// TODO: Check that the value is a valid algorithm
impl FlagInput for Algorithm {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError> {
        Ok(Setting::HashType(value.to_owned()))
    }
}
