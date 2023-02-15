use crate::cli::flag::get_help;

pub fn print_help(flag: &str) {
    let f = get_help(flag).unwrap();
    println!("{}", f.help());
    std::process::exit(0);
}
