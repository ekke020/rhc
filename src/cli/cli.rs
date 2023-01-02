use super::{arg::Arg, argument_error::NO_ARGUMENT_ERROR};
use std::{collections::HashMap, fmt::format, process, rc::Rc};

pub struct Cli {
    title: String,
    version: String,
    usage: String,
    arguments: HashMap<char, Arg>,
    options: Rc<Vec<String>>,
}

impl Cli {
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
        self.arguments.insert(arg.get_short_name(), arg);
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
        match args.get(0).ok_or_else(|| NO_ARGUMENT_ERROR) {
            Ok(_) => parse(args, 0),
            Err(err) => err.exit(0x40),
        };
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


fn parse(args: Vec<String>, index: usize) {
    // TODO: Implement logic for argument parsing
    // A CLI can have multiple arguments & arguments can have their own CLIs
}