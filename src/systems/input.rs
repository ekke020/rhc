use crate::core::prelude::*;
use crate::systems::config::Config;

// pub fn take() -> PasswordInfo {
//     let args = cli::create_flag_matcher();
//     let config = Config::from(args);
//     config.scan();
//     PasswordInfoBuilder::new(config.get_hash())
//         .set_algorithm_type(config.get_algorithm_type())
//         .set_password_length(config.get_length())
//         .build()
// }

// mod cli {
//     use clap::{value_parser, Arg, ArgMatches, Command};

//     pub fn create_flag_matcher() -> ArgMatches {
//         Command::new("rhc")
//             .about("A password cracking tool that utilizes brute force to crack passwords.")
//             .arg(create_all_arg())
//             .arg(create_input_arg())
//             .arg(create_type_arg())
//             .arg(create_length_arg())
//             .get_matches()
//     }

//     fn create_all_arg() -> Arg<'static> {
//         Arg::new("all")
//             .required(false)
//             .long("all")
//             .short('a')
//             .help("List available algorithms")
//     }

//     fn create_input_arg() -> Arg<'static> {
//         Arg::new("input")
//             .required(false)
//             .takes_value(true)
//             .long("input")
//             .short('i')
//             .value_parser(value_parser!(String))
//             .help("the supplied hash")
//     }

//     fn create_type_arg() -> Arg<'static> {
//         Arg::new("type")
//             .required(false)
//             .takes_value(true)
//             .value_parser(value_parser!(String))
//             .long("type")
//             .short('t')
//             .help("Specifies the algorithm used to generate the hash")
//     }

//     fn create_length_arg() -> Arg<'static> {
//         Arg::new("length")
//             .required(false)
//             .takes_value(true)
//             .value_parser(value_parser!(usize))
//             .long("length")
//             .short('l')
//             .help("The length of the hashed value.")
//     }
// }
