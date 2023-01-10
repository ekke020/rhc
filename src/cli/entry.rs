use std::{collections::VecDeque, env};

use regex::Regex;

use super::{errors::argument_error::{
    ArgumentError, INVALID_ARGUMENT_ERROR, MALFORMED_ARGUMENT_ERROR, NO_ARGUMENT_ERROR,
}, flag::{FlagInfo, Flag, FlagType}};

pub fn entry() -> VecDeque<String> {
    let mut args = env::args()
        .into_iter()
        .skip(1)
        .collect::<VecDeque<String>>();
    args.get(0).is_none().then(|| NO_ARGUMENT_ERROR.exit(0x40));
    args
}

pub fn parse_args(mut args: VecDeque<String>) -> Result<Vec<FlagInfo>, ArgumentError> {
    let mut flags: Vec<FlagInfo> = vec![FlagInfo::from(Flag::Help)];
    let mut previous_flag = flags.last_mut();
    while !args.is_empty() {
        let arg = args.pop_front().unwrap();
        let parsed = parse_value(&arg)?;

        match parsed {
            FlagType::Option(v) => {
                let flag = match_flag(&v)?;
                flags.push(FlagInfo::from(flag));
                previous_flag = flags.last_mut();
            }
            FlagType::Input(v) => {
                let flaginfo = previous_flag.as_mut().ok_or(MALFORMED_ARGUMENT_ERROR)?;
                flaginfo.set_input(Some(v));
                previous_flag = None;
            }
            FlagType::Toggle => {
                let flaginfo = previous_flag.as_mut().ok_or(INVALID_ARGUMENT_ERROR)?;
                flaginfo.toggle_help();
            }
        };
    }
    Ok(flags)
}

fn match_flag(arg: &str) -> Result<Flag, ArgumentError> {
    match arg {
        "-h" | "--help" => Ok(Flag::Help),
        "-v" | "--version" => Ok(Flag::Version),
        "-i" | "--input" => Ok(Flag::Input),
        "-t" | "--type" => Ok(Flag::Type),
        "-l" | "--length" => Ok(Flag::Length),
        _ => Err(INVALID_ARGUMENT_ERROR),
    }
}

pub fn parse_value(value: &str) -> Result<FlagType, ArgumentError> {
    let input = Regex::new(r"^[aA-zZ]+").unwrap();
    let option = Regex::new(r"^--?[aA-zZ]+").unwrap();
    let help = Regex::new(r"^--help|^-h").unwrap();

    if let Some(v) = help.find(value) {
        return Ok(FlagType::Toggle);
    } else if let Some(v) = option.find(value) {
        return Ok(FlagType::Option(v.as_str().to_owned()));
    } else if let Some(v) = input.find(value) {
        return Ok(FlagType::Input(v.as_str().to_owned()));
    }

    Err(INVALID_ARGUMENT_ERROR)
}
