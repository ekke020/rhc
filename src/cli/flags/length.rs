use crate::cli::{
    error::argument::{ArgumentError, INVALID_INPUT_ERROR},
    settings::Setting,
};

use super::{FlagHelp, FlagInfo, FlagInput};

const SHORTHAND: char = 'l';
const NAME: &str = "length";
const SHORT_HELP: &str = "The length to start generating words from";
const LONG_HELP: &str = "The input should be an unsigned 32 bit integer.
The length flag is used to start the tool from a specified word length.
Example: rhc [OPTIONS]... -l 10
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
        Ok(Setting::HashLenght(length))
    }
}
