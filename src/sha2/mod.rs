mod bit_utils;
mod consts;
mod sha256_core;
mod sha512_core;
mod implementation;
mod wrapper;
mod testing;

use self::implementation::{U28, U32, U48, U64, U64_32, U64_28};
use self::sha256_core::{Sha224 as ShaCore224, Sha256 as ShaCore256};
use self::sha512_core::{Sha384 as ShaCore384, Sha512 as ShaCore512, Sha512_256 as ShaCore512_256, Sha512_224 as ShaCore512_224};
use self::wrapper::Wrapper;

pub type Sha224 = Wrapper<ShaCore224, U28>;
pub type Sha256 = Wrapper<ShaCore256, U32>;
pub type Sha384 = Wrapper<ShaCore384, U48>;
// pub type Sha512 = Wrapper<ShaCore512, U64>;
pub type Sha512_224 = Wrapper<ShaCore512_224, U64_28>;
pub type Sha512_256 = Wrapper<ShaCore512_256, U64_32>;
pub type Testing = testing::Test<ShaCore512, testing::U64, 64>;

pub fn convert_to_decimal_array(hash: &str) -> Vec<u32> {
    hash
        .chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .iter()
        .map(|s| u32::from_str_radix(s, 16).unwrap())
        .collect::<Vec<u32>>()
}