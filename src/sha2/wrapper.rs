use super::{
    bit_utils::lazy_vector,
    consts::{ByteSize, BYTE_SIZE_128, BYTE_SIZE_64},
    implementation::{CompressionSize, Extract, Hash, Sha},
};
use std::marker::PhantomData;

pub struct Wrapper<T, U> {
    sha2: T,
    compression: PhantomData<U>,
}
impl<T, U> Wrapper<T, U>
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

impl<T, U> Wrapper<T, U>
where
    T: Hash<U>,
    U: CompressionSize + Extract,
{
    pub fn extract(&mut self) -> Vec<<U as Extract>::Size> {
        let value = self.sha2.extract();
        value.take()
    }

    pub fn extract_as_lower_hex(&mut self) -> String {
        self.extract().iter()
            .map(|dec| format!("{:x}", dec))
            .collect()
        // value.iter().map(|dec| format!("{:01$x}", dec, 16)).collect()
    }

    pub fn extract_as_upper_hex(&mut self) -> String {
        self.extract().iter()
            .map(|dec| format!("{:X}", dec))
            .collect()
    }
}

impl<T, U> Wrapper<T, U>
where
    T: Sha,
    U: Extract,
{
    pub fn new(data: impl AsRef<[u8]>) -> Self {
        Self {
            sha2: T::new(data.as_ref()),
            compression: PhantomData,
        }
    }
}
