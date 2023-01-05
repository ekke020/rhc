mod builder;
mod errors;
use std::env;
use builder::cli_builder::CliBuilder;
pub fn entrypoint() {
    let cli = CliBuilder::new("test title", Some("0.0.1")).build();
    let args = env::args().into_iter().skip(1).collect::<Vec<String>>();
    cli.run(args);
}
