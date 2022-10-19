mod bit_utils;
mod consts;
mod sha256_core;
mod implementation;
mod wrapper;
use self::implementation::{U32, U28};
use self::sha256_core::{Sha224 as ShaCore224, Sha256 as ShaCore256};
use self::wrapper::Wrapper;

pub type Sha256 = Wrapper<ShaCore256, U32>;
pub type Sha224 = Wrapper<ShaCore224, U28>;
