use super::{FlagInfo, FlagHelp, FlagInput};
use crate::cli::{settings::Setting, error::{flag::FlagError, argument::ArgumentError}};

const SHORTHAND: char = 'p';
const NAME: &str = "--password";
const SHORT_HELP: &str = "The hashed word";
const LONG_HELP: &str = "Flag: -p | --password
Details:
    input type: string (hashed word)
Description:
    The --password flag is used to provide the hashed value of the password for cracking. 
    The input should be the hashed value, not the plaintext password. 
    To ensure compatibility, it is recommended to check the list of supported algorithms 
    using the --algorithm flag before providing a hashed value.
Example: 
    rhc [OPTIONS]... -p 90a3ed9e32b2aaf4c61c410eb925426119e1a9dc53d4286ade99a809
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