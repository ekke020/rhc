use super::{FlagInfo, FlagHelp, FlagInput};
use crate::cli::{settings::Setting, error::{flag::FlagError, argument::{ArgumentError, MALFORMED_HASH_ERROR}}};

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
// TODO: Rename this to target?
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
 // TODO: Should validate right here instead
impl FlagInput for Password {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError> {
        let parsed = hex_string_to_bytes(value)?;
        Ok(Setting::HashInput(parsed))
    }
}

fn hex_string_to_bytes(hex_string: &str) -> Result<Vec<u8>, ArgumentError> {
    let hex_values = hex_string.as_bytes();
    let mut result = Vec::new();
    if hex_values.len() % 2 != 0 {
        return Err(MALFORMED_HASH_ERROR)
    }
    for i in (0..hex_values.len()).step_by(2) {
        let hex_value = &hex_values[i..i + 2];
        let value = u8::from_str_radix(std::str::from_utf8(hex_value).unwrap(), 16)?;
        result.push(value);
    }
    Ok(result)
}