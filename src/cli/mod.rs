mod arg;
mod argument_error;
mod cli;
use std::env;

pub fn entrypoint() {
    let cli = cli::Cli::new("test title", "0.0.1");
    let args = env::args().into_iter().skip(1).collect::<Vec<String>>();
    // cli.run(args);
    println!("{}", cli.get_help());
}
