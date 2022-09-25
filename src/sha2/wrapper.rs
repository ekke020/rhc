pub struct Sha256;
pub struct Sha512;
pub trait Hash<const T: usize> {
    fn reload();

    fn run(&mut self);

    fn extract() -> [u32; T];

    fn compression<const N: usize>(mutated: [u32; N]) -> T;

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
