mod base_256;
mod base_512;
mod consts;
mod wrapper;
mod sha256_core;
use wrapper::Wrapper;
use wrapper::var_core::*;
// TODO: Wrap this with something so it can access consts?
// TODO: Maybe take a whole new direction?
// Make seperate structs and implementations for 224 & 256?
pub type Sha256<'a> = sha256_core::Sha256<'a>;
// pub type Sha224<'a> = Wrapper<sha256_core::Sha256<'a>, VarCore>;
