mod builder;
pub mod errors;
use builder::cli_builder::CliBuilder;
use std::collections::VecDeque;
use std::env;
mod argument;
use self::arg::Arg;
pub mod arg;




pub fn entrypoint() {
    let mut cli = CliBuilder::new("test title", Some("0.0.1")).build();
    let args = env::args().into_iter().skip(1).collect::<Vec<String>>();
    let mut args = VecDeque::from(args);

    match cli.run(&mut args) {
        Ok(options) => test(options),
        Err(e) => e.exit(0x40),
    }
}

fn test(options: Vec<&Arg>) {
    let ptr = options.as_ptr();
    for i in 0..options.len() {
        unsafe {
            println!("Element {} is pointing towards {:?}", i, *ptr.offset(i.try_into().unwrap()));
        }
    }
}
