use crate::cli::flags::get_help;

pub fn print_help() {
    let f = get_help("--help").unwrap();
    println!("{}", f.help());
    std::process::exit(0);
}
