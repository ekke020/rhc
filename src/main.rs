#![allow(warnings)]
mod cli;
mod core;
mod sha2;
mod systems;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::env;
use std::str::FromStr;

use cli::arg::Arg;
use cli::errors::argument_error::{
    ArgumentError, INVALID_ARGUMENT_ERROR, MALFORMED_ARGUMENT_ERROR, NO_ARGUMENT_ERROR,
};
use regex::Regex;

use crate::systems::printer::print;

trait argument<'help> {
    const SHORTHAND: Option<char>;
    const ARG_TYPE: ArgumentType;
    const HELP: Option<&'help str>;
    fn get_name(&self) -> &str;
    fn get_help(&self) -> Option<&str>;
    fn get_shorthand(&self) -> Option<&char>;
}

enum ArgumentType {
    Input(String),
    Empty,
    Informational,
}

struct HelpArgument;
struct HashInputArgument;

impl<'help> argument<'help> for HelpArgument {
    const SHORTHAND: Option<char> = Some('h');
    const ARG_TYPE: ArgumentType = ArgumentType::Informational;
    const HELP: Option<&'help str> = Some("help");

    fn get_name(&self) -> &str {
        "help"
    }

    fn get_help(&self) -> Option<&str> {
        Self::HELP
    }

    fn get_shorthand(&self) -> Option<&char> {
        Self::SHORTHAND.as_ref()
    }
}


fn main() {
    let test = Char::from('a');
    println!("{:?}", test.1);
    'e' as u8;
    // let args = env::args().into_iter().skip(1).collect::<Vec<String>>();
    // let mut args = VecDeque::from(args);
    // let mut flag = args.pop_front().ok_or_else(|| NO_ARGUMENT_ERROR);

    // if let Err(e) = flag.as_ref() {
    //     e.exit(0x40);
    // };

    // let mut option = String::new();
    // match parse_flag(flag.unwrap()) {
    //     Ok(arg) => option = arg,
    //     Err(e) => e.exit(0x40),
    // }
    // match option.as_str() {
    //     "h" | "help" => println!("{}", describe(HelpArgument)),
    //     _ => panic!("THIS SHOULD NOT HAPPEN"),
    // }
}

fn parse_flag(flag: String) -> Result<String, ArgumentError<'static>> {
    let re = Regex::new(r"^--?").unwrap();
    re.is_match(&flag)
        .then(|| true)
        .ok_or_else(|| MALFORMED_ARGUMENT_ERROR)?;

    let re = Regex::new(r"[aA-zZ]+").unwrap();
    let option = re
        .find(&flag)
        .ok_or_else(|| INVALID_ARGUMENT_ERROR)?
        .as_str()
        .to_owned();

    Ok(option)
}

fn describe<'a, T: argument<'a>>(arg: T) -> String {
    let mut help_text = String::from("\t\t");
    let mut short_name = String::from("    ");
    if let Some(text) = arg.get_help() {
        help_text.push_str(text);
    }
    if let Some(shorthand) = arg.get_shorthand() {
        short_name = format!(" -{},", shorthand);
    };
    format!("{} --{} {}", short_name, arg.get_name(), help_text)
}


