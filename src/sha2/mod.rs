mod bit_utils;
mod consts;
mod sha256_core;
mod wrapper;
mod sha_wrapper;
// TODO: Wrap sha256_core and create helper functions so not to exposed the underlying API.
// Make seperate structs and implementations for 224 & 256?
pub type Sha256<'a> = sha_wrapper::ShaWrapper<sha256_core::Sha256<'a>>;
pub type Sha224<'a> = sha256_core::Sha224<'a>;
// pub type Sha224<'a> = Wrapper<sha256_core::Sha256<'a>, VarCore>;
