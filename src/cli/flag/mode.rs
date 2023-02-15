use crate::{
    algorithm::AlgorithmType,
    cli::{error::argument::ArgumentError, settings::Setting}, core::crack::mode,
};

use super::{FlagHelp, FlagInfo, FlagInput};

const NAME: &str = "mode";
const SHORT_HELP: &str = "The cracking mode to use";
const LONG_HELP: &str = "Flag: --mode
Details:
    input type: string (specific mode to use)
Supported modes: 
    dictionary - A simple yet effective mode that target words included in a file.
    incremental - A powerful cracking mode that will try all possbile character combinations.
Description:
    The --mode flag specifies the cracking mode to use.
    This flag is optional as of now, the default behaviour is to start in the wordlist mode
    if a wordlist was provided and then continue in incremental mode.
Example: 
    rhc [OPTIONS]... --mode incremental 
";

pub(super) struct Mode;

impl FlagInfo for Mode {
    fn describe(&self) -> String {
        format!("    --{NAME} \t\t{SHORT_HELP}")
    }
}

impl FlagHelp for Mode {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagInput for Mode {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError> {
        let mode = mode::Mode::from(value)
            .ok_or_else(|| ArgumentError::unsupported_mode(value))?;
        Ok(Setting::Mode(mode))
    }
}
