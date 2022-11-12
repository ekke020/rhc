mod bit_utils;
mod consts;
mod sha256_core;
mod sha512_core;
mod implementation;
mod wrapper;

use self::implementation::{U28, U32, U48, U64};
use self::sha256_core::{Sha224 as ShaCore224, Sha256 as ShaCore256};
use self::sha512_core::{Sha384 as ShaCore384, Sha512 as ShaCore512, Sha512_256 as ShaCore512_256, Sha512_224 as ShaCore512_224};
use self::wrapper::Wrapper;

pub type Sha224 = Wrapper<ShaCore224, U28, 28>;
pub type Sha256 = Wrapper<ShaCore256, U32, 32>;
pub type Sha384 = Wrapper<ShaCore384, U48, 48>;
pub type Sha512 = Wrapper<ShaCore512, U64, 64>;
pub type Sha512_224 = Wrapper<ShaCore512_224, U28, 28>;
pub type Sha512_256 = Wrapper<ShaCore512_256, U32, 32>;

#[cfg(test)]
mod test {

    use super::*;
    const TEST: &str = "test";
    #[test]
    fn sha224() {
        let hashed_224_value_of_test = "90a3ed9e32b2aaf4c61c410eb925426119e1a9dc53d4286ade99a809";
        let mut sha224 = Sha224::new(TEST);
        sha224.run();
        assert_eq!(sha224.extract_as_lower_hex(), hashed_224_value_of_test);
    }
    #[test]
    fn sha256() {
        let hashed_256_value_of_test = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";
        let mut sha256 = Sha256::new(TEST);
        sha256.run();
        assert_eq!(sha256.extract_as_lower_hex(), hashed_256_value_of_test);
    }
    #[test]
    fn sha384() {
        let hashed_384_value_of_test = "768412320f7b0aa5812fce428dc4706b3cae50e02a64caa16a782249bfe8efc4b7ef1ccb126255d196047dfedf17a0a9";
        let mut sha384 = Sha384::new(TEST);
        sha384.run();
        assert_eq!(sha384.extract_as_lower_hex(), hashed_384_value_of_test);
    }
    #[test]
    fn sha512() {
        let hashed_512_value_of_test = "ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff";
        let mut sha512 = Sha512::new(TEST);
        sha512.run();
        assert_eq!(sha512.extract_as_lower_hex(), hashed_512_value_of_test);
    }
    #[test]
    fn sha512_224() {
        let hashed_512_224_value_of_test = "06001bf08dfb17d2b54925116823be230e98b5c6c278303bc4909a8c";
        let mut sha512_224 = Sha512_224::new(TEST);
        sha512_224.run();
        assert_eq!(sha512_224.extract_as_lower_hex(), hashed_512_224_value_of_test);
    }
    #[test]
    fn sha512_256() {
        let hashed_512_256_value_of_test = "3d37fe58435e0d87323dee4a2c1b339ef954de63716ee79f5747f94d974f913f";
        let mut sha512_256 = Sha512_256::new(TEST);
        sha512_256.run();
        assert_eq!(sha512_256.extract_as_lower_hex(), hashed_512_256_value_of_test);
    }
}