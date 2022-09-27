use std::convert::TryInto;

pub struct Sha256;
pub struct Sha512;
pub trait CompressionSize<T: Sized, const N: usize> {
    fn transform(compressed: [T; N]) -> Self;
}
pub struct U32([u32; 8]);
impl CompressionSize<u32, 8> for U32 {
    fn transform(compressed: [u32; 8]) -> Self {
        U32(compressed)
    }
}
pub struct U28([u32; 7]);
impl U28 {
    pub fn take(&mut self) -> [u32; 7] {
        self.0
    }
}
impl CompressionSize<u32, 8> for U28 {
    fn transform(compressed: [u32; 8]) -> Self {
        U28(compressed[0..7].try_into().unwrap())
    }
}

pub trait Hash<T: CompressionSize<u32, 8>> {
    fn reload();

    fn run(&mut self);

    fn extract(&mut self) -> T;

    // fn compression(mutated: [u32; 64]) -> T;
}

// pub struct Wrapper<T, Variables>
// where
//     T: Hash,
// {
//     core: T,
//     variables: Variables,
// }

pub mod var_core {
    use crate::sha2::consts::{State256, H256_224, H256_256};

    pub trait VariableOutPutCore {}

    pub type VarCore = ShaVarCore;

    pub const SHA_VAR_CORE_224: VarCore = ShaVarCore {
        size: 7,
        state: H256_224,
    };
    pub const SHA_VAR_CORE_256: VarCore = ShaVarCore {
        size: 8,
        state: H256_256,
    };

    pub struct ShaVarCore {
        size: usize,
        state: State256,
    }

    impl VariableOutPutCore for ShaVarCore {}
}
