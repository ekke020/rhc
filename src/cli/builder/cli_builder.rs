use crate::cli::arg::Arg;

use super:: cli::Cli;
use std::{collections::HashMap, rc::Rc};

pub struct CliBuilder {
    title: String,
    version: Option<String>,
    usage: String,
    arguments: Vec<Arg>,
}

impl CliBuilder {
    pub fn new(title: &str, version: Option<&str>) -> Self {
        Self::default().set_title(title).set_version(version)
    }

    fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    fn set_version(mut self, version: Option<&str>) -> Self {
        self.version = version
            .is_some()
            .then(|| version.unwrap())
            .map(|v| v.to_owned());
        self
    }

    pub fn add_arg(mut self, arg: Arg) -> Self {
        self.arguments.push(arg);
        self
    }

    pub fn build(self) -> Cli {
        let shorthands = evaluate_shorthands(&self.arguments);
        let options = evaluate_options(&self.arguments);
        let arguments = evaluate_names(self.arguments);
        Cli {
            title: self.title,
            version: self.version,
            usage: "todo!()".to_owned(),
            arguments,
            options,
            shorthands: Default::default(),
        }
    }
}

impl Default for CliBuilder {
    fn default() -> Self {
        Self {
            title: Default::default(),
            version: Default::default(),
            usage: Default::default(),
            arguments: vec![Arg::new("help")
                .short_name('h')
                .help("Print help information")],
        }
    }
}

fn evaluate_names(arguments: Vec<Arg>) -> HashMap<String, Arg> {
    let mut map = HashMap::new();

    let names = arguments
        .into_iter()
        .map(|arg| (arg.get_name().to_owned(), arg));

    for (name, arg) in names {
        if map.contains_key(&name) {
            panic!("Duplicate names are not allowed: found {}", name);
        }
        map.insert(name, arg);
    }
    map
}

fn evaluate_shorthands(arguments: &Vec<Arg>) -> HashMap<&char, String> {
    let mut shorthands = HashMap::new();

    let pairs = arguments
        .iter()
        .map(|arg| (arg.get_shorthand(), arg.get_name().to_owned()))
        .filter(|(char, _)| char.is_some())
        .map(|(short, name)| (short.unwrap(), name));

    for (shorthand, name) in pairs {
        if shorthands.contains_key(&shorthand) {
            panic!("Duplicate shorthands are not allowed: found {}", shorthand);
        }
        shorthands.insert(shorthand, name);
    };
    shorthands

}

fn evaluate_options(arguments: &Vec<Arg>) -> Rc<Vec<String>> {
    let mut options = Vec::new();
    for arg in arguments {
        options.push(format!("{}\n", arg.describe()));
    }
    Rc::new(options)
}