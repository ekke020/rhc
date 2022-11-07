#![allow(warnings)]
mod core;
mod sha2;
mod systems;
use std::convert::TryInto;
use std::env;

use crate::systems::printer::print;
fn main() {
    // let password_info = systems::input::take();
    // systems::spawner::run_threads(password_info);
    // gpu_test();
    let hashed_224_value_of_test = "90a3ed9e32b2aaf4c61c410eb925426119e1a9dc53d4286ade99a809";
    let hashed_256_value_of_test = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";
    let hashed_384_value_of_test = "768412320f7b0aa5812fce428dc4706b3cae50e02a64caa16a782249bfe8efc4b7ef1ccb126255d196047dfedf17a0a9";
    let hashed_512_value_of_test = "ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff";
    let hashed_512_224_value_of_test = "06001bf08dfb17d2b54925116823be230e98b5c6c278303bc4909a8c";
    let hashed_512_256_value_of_test = "3d37fe58435e0d87323dee4a2c1b339ef954de63716ee79f5747f94d974f913f";
    let test = "test";
    // let mut sha224 = sha2::Sha224::new(test);
    // sha224.run();
    // assert_eq!(sha224.extract_as_lower_hex(), hashed_224_value_of_test);
    // let mut sha256 = sha2::Sha256::new(test);
    // sha256.run();
    // assert_eq!(sha256.extract_as_lower_hex(), hashed_256_value_of_test);
    // let mut sha384 = sha2::Sha384::new(test);
    // sha384.run();
    // assert_eq!(sha384.extract_as_lower_hex(), hashed_384_value_of_test);
    let mut sha512 = sha2::Sha512::new(test);
    sha512.run();
    sha512.extract().iter().for_each(|c| print!("{:01$x?} ", c.to_be_bytes(), 2));
    // assert_eq!(sha512.extract_as_lower_hex(), hashed_512_value_of_test);
    // let mut sha512_256 = sha2::Sha512_256::new(test);
    // sha512_256.run();
    // assert_eq!(sha512_256.extract_as_lower_hex(), hashed_512_256_value_of_test);
    // let mut sha512_224 = sha2::Sha512_224::new(test);
    // sha512_224.run();
    // assert_eq!(sha512_224.extract_as_lower_hex(), hashed_512_224_value_of_test);
}