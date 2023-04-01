use super::{FlagHelp, FlagInfo, FLAG_DESCRIPTIONS};
use crate::cli::argument::VERSION;

const SHORTHAND: char = 'h';
const NAME: &str = "help";
const SHORT_HELP: &str = "Shows extended help information";
const LONG_HELP: &str = "RHC
Version: {VERSION}
About:
    RHC is an open-source password cracking tool that is inspired by John the Ripper.
    As of version 0.1.0 RHC includes the SHA-2 family of algorithms, 
    and is designed to be fast, efficient, and highly configurable.

    It comes with three different character sets and allows you to set the starting 
    and ending length of targeted words, giving you fine-grained control over the cracking process. 
    It includes both a Dictionary mode and an Incremental mode, 
    enabling you to use pre-built wordlists or generate words on the fly.

    By providing a powerful and flexible password cracking solution, 
    RHC can help users test the strength of their passwords and improve 
    the security of their systems. As the author, I also gained valuable 
    experience and knowledge while developing RHC in Rust.

usage: rhc [OPTION]...
";

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
            .fold(LONG_HELP.replace("{VERSION}", VERSION), |a, b| a + "\n    " + &b.describe())
    }
}
