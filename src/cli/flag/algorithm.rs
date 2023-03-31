use super::*;
use crate::algorithm::AlgorithmType;

const NAME: &str = "algorithm";
const SHORT_HELP: &str = "The algorithm to target";
const LONG_HELP: &str = "Flag: --algorithm
Details:
    input type: string (specific algorithm to target)
Supported algorithms: 
    the SHA family:
        sha1, sha2_224, sha2_256, sha2_384,
        sha2_512, sha2_512_224, sha2_512_256
Description:
    The --algorithm flag specifies the algorithm to target when cracking a hash.
    The tool will attempt to automatically detect the algorithm used to create the hash, 
    but this may not always be possible. If the algorithm is known, 
    it is recommended to provide it as a flag to avoid the increased runtime and potential 
    false positives that come with trying multiple algorithms.
Example: 
    rhc [OPTIONS]... --algorithm sha2_224
";

pub(super) struct Algorithm;

impl FlagInfo for Algorithm {
    fn describe(&self) -> String {
        format!("    --{NAME} \t\t{SHORT_HELP}")
    }
}

impl FlagHelp for Algorithm {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagInput for Algorithm {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError> {
        let algorithm = AlgorithmType::from(value)
            .ok_or_else(|| ArgumentError::unsupported_algorithm(value))?;
        Ok(Setting::TargetType(algorithm))
    }
}
