use std::{collections::VecDeque, env};

use regex::Regex;

use super::{
    errors::argument::{
        ArgumentError, INVALID_ARGUMENT_ERROR, MALFORMED_ARGUMENT_ERROR, NO_ARGUMENT_ERROR,
    },
    flag::{Flag, FlagInfo, FlagType},
};

pub fn entry() -> Result<VecDeque<String>, ArgumentError> {
    let mut args = env::args()
        .into_iter()
        .skip(1)
        .collect::<VecDeque<String>>();
    args.get(0).ok_or(NO_ARGUMENT_ERROR)?;
    Ok(args)
}

pub fn parse_args(mut args: VecDeque<String>) -> Result<Vec<Flag>, ArgumentError> {
    let mut flags: Vec<Flag> = vec![Flag::from("--help").unwrap()];
    let mut previous_flag = flags.last_mut();
    while !args.is_empty() {
        let arg = args.pop_front().unwrap();
        let parsed = parse_value(&arg)?;

        match parsed {
            FlagType::Option(v) => {
                let flag = Flag::from(&v).ok_or(INVALID_ARGUMENT_ERROR)?;
                flags.push(flag);
                previous_flag = flags.last_mut();
            }
            FlagType::Input(v) => {
                let flag = previous_flag.as_mut().ok_or(MALFORMED_ARGUMENT_ERROR)?;
                flag.set_input(&v);
                previous_flag = None;
            }
            FlagType::Help => {
                let flaginfo = previous_flag.as_mut().ok_or(INVALID_ARGUMENT_ERROR)?;
                flaginfo.toggle_help();
            }
        };
    }
    Ok(flags)
}

fn parse_value(value: &str) -> Result<FlagType, ArgumentError> {
    let input = Regex::new(r"^[aA-zZ]+").unwrap();
    let option = Regex::new(r"^--?[aA-zZ]+").unwrap();
    let help = Regex::new(r"^--help|^-h").unwrap();

    if let Some(v) = help.find(value) {
        return Ok(FlagType::Help);
    } else if let Some(v) = option.find(value) {
        return Ok(FlagType::Option(v.as_str().to_owned()));
    } else if let Some(v) = input.find(value) {
        return Ok(FlagType::Input(v.as_str().to_owned()));
    }

    Err(INVALID_ARGUMENT_ERROR)
}
