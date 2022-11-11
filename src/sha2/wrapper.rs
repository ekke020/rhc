use super::{
    bit_utils::lazy_vector,
    consts::{ByteSize, BYTE_SIZE_128, BYTE_SIZE_64},
    implementation::{CompressionSize, Extract, Hash, Sha},
};
use std::marker::PhantomData;

pub struct Wrapper<T, U, const N: usize> {
    sha2: T,
    size: PhantomData<U>,
}
impl<T, U, const N: usize> Wrapper<T, U, N>
where
    T: Hash<U>,
    U: CompressionSize,
{
    pub fn run(&mut self) {
        self.sha2.run();
    }

    pub fn reload(&mut self, data: impl AsRef<[u8]>) {
        self.sha2.reload(data.as_ref());
    }
}

impl<T, U, const N: usize> Wrapper<T, U, N>
where
    T: Hash<U>,
    U: CompressionSize + Extract<N>
{
    pub fn extract(&mut self) -> [u8; N] {
        let sha = self.sha2.extract();
        sha.take()
        // &sha.take::<{ <U as Extract>::S }>()
        // let t = value.take::<{self.size}>();
        // t
    }

    pub fn extract_as_lower_hex(&mut self) -> String {
        self.extract()
            .iter()
            .map(|byte| format!("{:01$x}", byte, 2))
            .collect()
        // value.iter().map(|dec| format!("{:01$x}", dec, 16)).collect()
    }

    pub fn extract_as_upper_hex(&mut self) -> String {
        self.extract().iter()
            .map(|dec| format!("{:X}", dec))
            .collect()
    }
}

impl<T, U, const N: usize> Wrapper<T, U, N>
where
    T: Sha,
    // U: Extract,
{
    pub fn new(data: impl AsRef<[u8]>) -> Self {
        Self {
            sha2: T::new(data.as_ref()),
            size: PhantomData,
        }
    }
}
