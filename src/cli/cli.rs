use super::arg::Arg;
use std::collections::HashMap;

pub struct Cli {
    title: String,
    version: String,
    usage: String,
    arguments: HashMap<u64, Arg>,
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
            .add_arg(
                Arg::new("test3")
                    .help_text("This is yet another bit of a test"),
            )
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
        self.arguments.insert(arg.get_id(), arg);
        self
    }

    pub fn print_help(&self) {
        println!("{}", self.title);
        println!("Version: {}", self.version);
        println!("USAGE: \n\trhc [OPTIONS]...");
        println!("OPTIONS:");
        self.arguments.values().for_each(|arg| println!("{}", arg.describe()));
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            title: Default::default(),
            version: Default::default(),
            usage: Default::default(),
            arguments: Default::default(),
        }
    }
}
