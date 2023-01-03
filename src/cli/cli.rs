use super::{
    arg::Arg,
    argument_error::{ArgumentError, NO_ARGUMENT_ERROR, MALFORMED_ARGUMENT_ERROR, MISSING_INPUT_ERROR},
};
use std::{collections::HashMap, fmt::format, process, rc::Rc};
use regex::Regex;

pub struct Cli<'a> {
    title: String,
    version: String,
    usage: String,
    arguments: HashMap<&'a str, &'a Arg>,
    options: Rc<Vec<String>>,
}

impl <'a>Cli<'a> {
    pub fn new(title: &str, version: &str) -> Self {
        Self::default()
            .name(title.to_owned())
            .version(version.to_owned())
            .add_arg(
                Arg::new("help")
                    .short_name('h')
                    .help_text("Print help information"),
            )
            .add_arg(
                Arg::new("test")
                    .short_name('t')
                    .help_text("This is a bit of a test"),
            )
            .add_arg(
                Arg::new("test2")
                    .short_name('T')
                    .help_text("This is another bit of a test"),
            )
            .add_arg(Arg::new("test3").help_text("This is yet another bit of a test"))
    }

    fn name(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    fn version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    fn add_arg(mut self, arg: Arg) -> Self {
        let mut vec = Rc::make_mut(&mut self.options);
        vec.push(format!("{}\n", arg.describe()));
        self.arguments.insert(arg.get_name(), &arg);
        arg.has_shorthand().then(|| self.arguments.insert(arg.get_shorthand(), &arg));
        self
    }

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
Version: {}
USAGE: \n\trhc [OPTIONS]...
OPTIONS:
{}
        ",
            self.title, self.version, options
        )
    }

    pub fn run(&self, args: Vec<String>) {
        let do_steps = || -> Result<(), ArgumentError> {
            parse(args, 0)?;
            Ok(())
        };

        if let Err(err) = do_steps() {
            err.exit(0x40)
        }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            title: Default::default(),
            version: Default::default(),
            usage: Default::default(),
            arguments: Default::default(),
            options: Default::default(),
        }
    }
}

fn parse(args: Vec<String>, index: usize) -> Result<(), ArgumentError<'static>> {
    // TODO: Implement logic for argument parsing
    // A CLI can have multiple arguments & arguments can have their own CLIs
    let flag = args.get(index).ok_or_else(|| MALFORMED_ARGUMENT_ERROR)?;
    let re = Regex::new(r"^--?[aA-zZ]+").unwrap();
    let option_name = re
        .find(flag)
        .ok_or_else(|| NO_ARGUMENT_ERROR)?
        .as_str();
    println!("{}", option_name);
    Ok(())
}
