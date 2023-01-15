use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;


pub struct ArgumentInfo {
    help: &'static str,
    help_long: &'static str,
    shorthand: Option<char>,
    name: &'static str,
}
impl ArgumentInfo {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_help(&self) -> &str {
        self.help
    }

    fn get_shorthand(&self) -> Option<&char> {
        self.shorthand.as_ref()
    }
}

#[derive(Copy, Clone)]
pub struct Char(char, [u8; 4]);
impl Char {
    fn from(char: char) -> Self {
        let mut binding: [u8; 4] = [0; 4];
        char.encode_utf8(&mut binding);
        Self(char, binding)
    }
}

impl AsRef<[u8]> for Char {
    fn as_ref(&self) -> &[u8] {
        &self.1
    }
}

impl Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


fn match_shorthand(shorthand: &Char, flag: &[u8]) -> bool {
    shorthand.1 == flag
}

fn match_name(name: &str, flag: &[u8]) -> bool {
    name.as_bytes().eq(flag)
}

pub fn describe(arg: &ArgumentInfo) -> String {
    let mut short_name = String::from("    ");
    if let Some(shorthand) = arg.get_shorthand() {
        short_name = format!(" -{},", shorthand);
    };
    format!("{} --{} \t\t{}", short_name, arg.get_name(), arg.get_help())
}

pub mod info {
    use super::ArgumentInfo;

    pub const ARGUMENTS: [ArgumentInfo; 5] = [HELP_ARG, INPUT_ARG, TYPE_ARG, LENGTH_ARG, VERSION_ARG];

    pub const HELP: usize = 0;
    const HELP_MSG: &str = "INPUT TEST";
    const HELP_LONG_MSG: &str = "INPUT TEST";
    const HELP_NAME: &str = "help";
    const HELP_ARG: ArgumentInfo = ArgumentInfo {
        help: HELP_MSG,
        shorthand: Some('h'),
        name: HELP_NAME,
        help_long: HELP_LONG_MSG,
    };

    pub const INPUT: usize = 1;
    const INPUT_MSG: &str = "INPUT TEST";
    const INPUT_LONG_MSG: &str = "INPUT TEST";
    const INPUT_NAME: &str = "input";
    const INPUT_ARG: ArgumentInfo = ArgumentInfo {
        help: INPUT_MSG,
        shorthand: Some('i'),
        name: INPUT_NAME,
        help_long: INPUT_LONG_MSG,
    };

    pub const TYPE: usize = 2;
    const TYPE_MSG: &str = "INPUT TEST";
    const TYPE_LONG_MSG: &str = "INPUT TEST";
    const TYPE_NAME: &str = "type";
    const TYPE_ARG: ArgumentInfo = ArgumentInfo {
        help: TYPE_MSG,
        shorthand: Some('t'),
        name: TYPE_NAME,
        help_long: TYPE_LONG_MSG,
    };

    pub const LENGTH: usize = 3;
    const LENGTH_MSG: &str = "INPUT TEST";
    const LENGTH_LONG_MSG: &str = "INPUT TEST";
    const LENGTH_NAME: &str = "length";
    const LENGTH_ARG: ArgumentInfo = ArgumentInfo {
        help: LENGTH_MSG,
        shorthand: Some('l'),
        name: LENGTH_NAME,
        help_long: LENGTH_LONG_MSG,
    };

    pub const VERSION: usize = 4;
    const VERSION_MSG: &str = "INPUT TEST";
    const VERSION_LONG_MSG: &str = "INPUT TEST";
    const VERSION_NAME: &str = "version";
    const VERSION_ARG: ArgumentInfo = ArgumentInfo {
        help: VERSION_MSG,
        shorthand: Some('v'),
        name: VERSION_NAME,
        help_long: VERSION_LONG_MSG,
    };
}
