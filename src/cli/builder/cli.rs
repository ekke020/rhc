use regex::Regex;
use std::{collections::HashMap, fmt::format, process, rc::Rc};

use super::arg::Arg;
use crate::cli::errors::argument_error::{
    ArgumentError, INVALID_ARGUMENT_ERROR, MALFORMED_ARGUMENT_ERROR, NO_ARGUMENT_ERROR,
};

pub struct Cli {
    pub title: String,
    pub version: Option<String>,
    pub usage: String,
    pub arguments: HashMap<String, Arg>,
    pub shorthands: HashMap<char, String>,
    pub options: Rc<Vec<String>>,
}

pub trait cliArguments {
    fn has_argument(&self) -> bool;

    fn parse(args: Vec<String>, index: usize) -> Result<Arg, ArgumentError<'static>>;
}

impl Cli {
    pub fn get_help(&self) -> String {
        let options = self
            .options
            .iter()
            .map(|s| s.clone())
            .reduce(|o, s| format!("{}{}", o, s))
            .unwrap();
        format!(
            "
{}
USAGE: \n\trhc [OPTIONS]...
OPTIONS:
{}
        ",
            self.title, options
        )
    }

    pub fn run(&self, args: Vec<String>) {
        let do_steps = || -> Result<(), ArgumentError> {
            let (arg, index) = parse_arg(args, 0)?;
            let argument = self.lookup_arg(arg)?;
            println!("{}", argument.describe());
            // TODO: Check if the args vector is empty and stop parsing.
            // TODO: Send args vector over to the next argument
            Ok(())
        };

        if let Err(err) = do_steps() {
            err.exit(0x40)
        }
    }

    fn lookup_arg(&self, arg: String) -> Result<&Arg, ArgumentError> {
        let argument = if arg.len() == 1 {
            let char = arg.chars().next().unwrap();
            self.shorthands
                .get(&char)
                .ok_or_else(|| ArgumentError::custom(format!("Invalid shorthand argument: {}", char), true))?
                .as_str()
        } else {
            &arg
        };
        let value = self
            .arguments
            .get(argument)
            .ok_or_else(|| ArgumentError::custom(format!("Invalid argument: {}", argument), true))?;

        Ok(value)
    }
}
impl Default for Cli {
    fn default() -> Self {
        Self {
            title: Default::default(),
            version: Default::default(),
            usage: Default::default(),
            arguments: Default::default(),
            shorthands: Default::default(),
            options: Default::default(),
        }
    }
}

fn parse_arg(args: Vec<String>, index: usize) -> Result<(String, usize), ArgumentError<'static>> {
    // TODO: Make the args vector mutable and remove each parsed value.
    let flag = args.get(index).ok_or_else(|| NO_ARGUMENT_ERROR)?;
    let re = Regex::new(r"^--?").unwrap();
    re.is_match(flag)
        .then(|| true)
        .ok_or_else(|| MALFORMED_ARGUMENT_ERROR)?;

    let re = Regex::new(r"[aA-zZ]+").unwrap();
    let option = re
        .find(flag)
        .ok_or_else(|| INVALID_ARGUMENT_ERROR)?
        .as_str()
        .to_owned();

    Ok((option, index + 1))
}
