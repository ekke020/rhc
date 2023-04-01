use crate::cli::flag::get_help;

pub fn print_help(flag: &str) {
    let f = get_help(flag);
    let exit_code = match get_help(flag) {
        Ok(f) => {
            println!("{}", f.help());
            0
        }
        Err(err) => {
            println!("{}", err);
            err.get_exit_code()
        }
    };
    std::process::exit(exit_code);
}
