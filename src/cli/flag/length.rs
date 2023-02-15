use crate::cli::{
    error::argument::{ArgumentError, INVALID_INPUT_ERROR},
    settings::Setting,
};

use super::{FlagHelp, FlagInfo, FlagInput};

const SHORTHAND: char = 'l';
const NAME: &str = "length";
const SHORT_HELP: &str = "The length to start generating words from";
const LONG_HELP: &str = "Flag: -l | --length
Details:
    input type: unsigned 32 bit integer
Description:
    The --length flag is used to specify the starting length of words 
    to generate when cracking a hash. 
    It is important to note that this flag only sets the starting length
    and does not determine the target length. To set the maximum target length, 
    use the --max-length flag [not yet specified].
Example: 
    rhc [OPTIONS]... -l 10
";

pub(super) struct Length;

impl FlagInfo for Length {
    fn describe(&self) -> String {
        format!("-{SHORTHAND}, --{NAME} \t\t{SHORT_HELP}")
    }
}

impl FlagHelp for Length {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagInput for Length {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError> {
        let length = value.parse::<u32>().ok().ok_or(INVALID_INPUT_ERROR)?;
        Ok(Setting::HashLength(length))
    }
}
