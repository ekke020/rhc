pub const VERSION: &str = "0.0.1";

pub fn print_version() {
    println!("rhc version: {}", VERSION);
    std::process::exit(0);
}
