use super::{FlagHelp, FlagInfo, FlagInput};
use crate::{core::charset::CharacterSet, cli::{settings::Setting, error::argument::ArgumentError}};
const SHORTHAND: char = 'c'; 
const NAME: &str = "charset";
const SHORT_HELP: &str = "The charset to use";
const LONG_HELP: &str = "Flag: -c | --charset
Details:
    input type: string (charset to use)
Supported charsets: 
    ASCII_95 - Represents a 95 character set with letters, digits and symbols.        
    COMMON - Represents the 77 most common characters used in passwords.         
    NO_SPECIAL - Represents 66 characters that are only made up of digits and letters.                
Description:
    Use the --charset flag to specify the charset to use when cracking a hash. 
    The charset is the set of characters that rhc will use when trying to guess 
    the password for a given hash. The COMMON charset is used by default if no 
    charset is specified.
Example: 
    rhc [OPTIONS]... --charset ASCII_95
";

pub(super) struct Charset;

impl FlagInfo for Charset {
    fn describe(&self) -> String {
        format!("-{SHORTHAND}, --{NAME} \t\t{SHORT_HELP}")
    }
}

impl FlagHelp for Charset {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagInput for Charset {
    fn produce_input_setting(&self, value: &str) -> Result<Setting, ArgumentError> {
        let charset = CharacterSet::from(value)
            .ok_or_else(|| ArgumentError::unsupported_character_set(value))?;
        Ok(Setting::Charset(charset))
    }
}
