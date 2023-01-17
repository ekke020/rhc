use crate::cli::settings::Setting;

use super::{FlagHelp, FlagInfo, FlagToggle, FLAG_DESCRIPTIONS};

const SHORTHAND: char = 'v';
const NAME: &str = "verbose";
const SHORT_HELP: &str = "Enables verbose output";
const LONG_HELP: &str = "Flag: -v | --verbose
Details:
    Toggles the verbose output
Description:
    Verbose output [not yet specified]...
Example: 
    rhc [OPTIONS]... -v 
";
pub(super) struct Verbose;

impl FlagInfo for Verbose {
    fn describe(&self) -> String {
        format!("-{SHORTHAND}, --{NAME} \t\t{SHORT_HELP}")
    }
}

impl FlagHelp for Verbose {
    fn help(&self) -> String {
        LONG_HELP.to_owned()
    }
}

impl FlagToggle for Verbose {
    fn produce_toggle_setting(&self) -> Setting {
        Setting::Verbose(true)
    }
}
