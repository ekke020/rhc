use clap::{ArgMatches, Command};

mod create {
    use clap::{value_parser, Arg};
    pub const FLAGS: [Arg; 4] = [
        Arg::new("length")
            .required(false)
            .takes_value(true)
            .value_parser(value_parser!(usize))
            .long("length")
            .short('l')
            .help("The length of the hashed value."),
        Arg::new("type")
            .required(false)
            .takes_value(true)
            .value_parser(value_parser!(String))
            .long("type")
            .short('t')
            .help("Specifies the algorithm used to generate the hash"),
        Arg::new("all")
            .required(false)
            .long("all")
            .short('a')
            .help("List available algorithms"),
        Arg::new("input")
            .required(false)
            .takes_value(true)
            .long("input")
            .short('i')
            .value_parser(value_parser!(String))
            .help("the supplied hash"),
    ];
}
pub fn create_flag_matcher() -> ArgMatches {
    Command::new("rhc")
        .about("A password cracking tool that utilizes brute force to crack passwords.")
        .arg(create::FLAGS[0])
        .arg(create::FLAGS[1])
        .arg(create::FLAGS[2])
        .arg(create::FLAGS[3])
        .get_matches()
}
