mod bit_utils;
mod consts;
mod sha256_core;
mod wrapper;
mod sha_wrapper;
use self::wrapper::U32;

// TODO: Wrap sha256_core and create helper functions so not to exposed the underlying API.
// Make seperate structs and implementations for 224 & 256?
use super::sha2::sha256_core::{Sha224 as ShaCore224, Sha256 as ShaCore256};
use super::sha2::sha_wrapper::ShaWrapper;
pub type Sha256 = ShaWrapper<ShaCore256, U32>;
pub type Sha224<'a> = sha256_core::Sha224<'a>;
// pub type Sha224<'a> = Wrapper<sha256_core::Sha256<'a>, VarCore>;
