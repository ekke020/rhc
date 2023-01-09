use regex::Regex;

use super::errors::argument_error::{ArgumentError, MALFORMED_ARGUMENT_ERROR, INVALID_ARGUMENT_ERROR};

enum ArgumentType {
    Input(String),
    Empty,
    Informational,
}

trait argument<'help> {
    const SHORTHAND: Option<char>;
    const ARG_TYPE: ArgumentType;
    const HELP: Option<&'help str>;
    fn get_name(&self) -> &str;
    fn get_help(&self) -> Option<&str>;
    fn get_shorthand(&self) -> Option<&char>;
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

#[derive(Copy, Clone)]
struct Char(char, [u8; 4]);
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


// TODO: Fix this compare function so it can take a &str and a Char type
fn is_match<'a, T: argument<'a>>(t: T, arg: impl AsRef<[u8]> + Into<Option<&'a [u8]>>) -> bool {
    // arg.map_or(false, |x| x.contains(t.as_ref()))
    false
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