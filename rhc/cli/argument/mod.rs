mod help;
mod version;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn help_and_exit(arg: Option<&str>) {
    let flag = arg.unwrap_or("--help");
    help::print_help(flag);
}

pub fn version_and_exit() {
    version::print_version();
}