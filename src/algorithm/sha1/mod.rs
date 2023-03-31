use std::fmt::Display;

use self::sha_core::ShaCore160;

use super::{compression::U20, wrapper::Wrapper};

mod consts;
mod sha_core;

pub type Sha160 = Wrapper<ShaCore160, U20, 20>;

impl Display for Sha160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", "Sha1")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sha_produces_correct_value() {
        let data = [97, 98, 99];
        let mut sha = Sha160::from(&data);
        sha.run();
        assert_eq!(sha.extract_as_lower_hex(), "a9993e364706816aba3e25717850c26c9cd0d89d");
    }
}