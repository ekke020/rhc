mod help;
mod version;

pub const VERSION: &str = "0.0.1";

pub fn help_and_exit(arg: Option<&str>) {
    let flag = arg.unwrap_or("--help");
    help::print_help(flag);
}

pub fn version_and_exit() {
    version::print_version();
}