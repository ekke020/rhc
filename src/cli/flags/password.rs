use super::{FlagInfo, FlagHelp, FlagInput};
use crate::cli::{settings::Setting, error::{flag::FlagError, argument::ArgumentError}};

const SHORTHAND: char = 'p';
pub const NAME: &str = "--password";
const SHORT_HELP: &str = "The hashed word";
const LONG_HELP: &str = "The input should be a hashed value from a supported algorithm.
See the --algorithm help for available algorithms.
Example: rhc -i 90a3ed9e32b2aaf4c61c410eb925426119e1a9dc53d4286ade99a809
";

pub(super) struct Password;

impl FlagInfo for Password {
    fn describe(&self) -> String {
        format!("-{SHORTHAND}, {NAME} \t\t{SHORT_HELP}")
    }
}

impl FlagHelp for Password {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagInput for Password {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError> {
        Ok(Setting::HashInput(value.to_owned()))
    }
}