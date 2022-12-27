mod arg;
mod cli;

pub fn entrypoint() {
    let cli = cli::Cli::new("test title", "0.0.1");
    cli.print_help();
}
