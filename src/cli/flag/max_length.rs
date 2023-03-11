use crate::cli::{
    error::argument::{ArgumentError, INVALID_INPUT_ERROR},
    settings::Setting,
};

use super::{FlagHelp, FlagInfo, FlagInput};

const NAME: &str = "max-length";
const SHORT_HELP: &str = "Specifies the maximum length to target";
const LONG_HELP: &str = "Flag: --max-length
Details:
    input type: unsigned 32 bit integer
Description:
    The --max-length flag is used to specify the maximum length of words 
    to generate when cracking a hash. 
    Note that the value specified with the --max-length flag must exceed the 
    value of min-length if the --min-length flag was specified.
Example: 
    rhc [OPTIONS]... --max-length 15
";

pub(super) struct MaxLength;

impl FlagInfo for MaxLength {
    fn describe(&self) -> String {
        format!("    --{NAME} \t\t{SHORT_HELP}")
    }
}

impl FlagHelp for MaxLength {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagInput for MaxLength {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError> {
        let length = value.parse::<usize>().ok().ok_or(INVALID_INPUT_ERROR)?;
        Ok(Setting::MaxLength(length))
    }
}
