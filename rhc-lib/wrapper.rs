use std::marker::PhantomData;

use super::compression::{CompressionSize, Hash, Sha, Extract};

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

    pub fn load(&mut self, data: impl AsRef<[u8]>) {
        self.sha2.load(data.as_ref());
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
    }

    pub fn extract_as_lower_hex(&mut self) -> String {
        self.extract()
            .iter()
            .map(|byte| format!("{:01$x}", byte, 2))
            .collect()
    }

    pub fn extract_as_upper_hex(&mut self) -> String {
        self.extract()
            .iter()
            .map(|dec| format!("{:X}", dec))
            .collect()
    }
}

impl<T, U, const N: usize> Wrapper<T, U, N>
where
    T: Sha,
{   
    pub fn new() -> Self {
        Self {
            sha2: T::new(),
            size: PhantomData,
        }
    }
    pub fn from(data: impl AsRef<[u8]>) -> Self {
        Self {
            sha2: T::from(data.as_ref()),
            size: PhantomData,
        }
    }
}
