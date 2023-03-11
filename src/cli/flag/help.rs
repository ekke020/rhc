use super::{FlagHelp, FlagInfo, FLAG_DESCRIPTIONS};

const SHORTHAND: char = 'h';
const NAME: &str = "help";
const SHORT_HELP: &str = "Shows extended help information";
const LONG_HELP: &str = "RHC
Version: 0.0.1
About:
    RHC is an implementation of John the ripper written in Rust
usage: rhc [OPTION]...";

pub(super) struct Help;

impl FlagInfo for Help {
    fn describe(&self) -> String {
        format!("-{SHORTHAND}, --{NAME} \t\t\t{SHORT_HELP}")
    }
}

impl FlagHelp for Help {
    fn help(&self) -> String {
        FLAG_DESCRIPTIONS
            .iter()
            .fold(String::from(LONG_HELP), |a, b| a + "\n    " + &b.describe())
    }
}
