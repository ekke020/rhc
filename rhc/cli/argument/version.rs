use crate::cli::argument::VERSION;

pub fn print_version() {
    println!("rhc version: {}", VERSION);
    std::process::exit(0);
}
