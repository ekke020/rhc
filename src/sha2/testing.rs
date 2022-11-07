use std::{convert::TryInto, fmt::{LowerHex, UpperHex}};
use super::consts::{ByteSize, BYTE_SIZE_64, BYTE_SIZE_128};

pub trait CompressionSize {
    type Size: Sized;
    fn transform(compressed: [Self::Size; 8]) -> Self;
}
pub trait Extract {
    type Size: Sized + LowerHex + UpperHex;
    const N: usize;
    // TODO: Come up with a way to make the size of the returned array generic.
    // fn take(self) -> [u8; Self::N];
    fn take(self) -> [u8; 64];
}
pub struct U64([u8; 64]);
impl CompressionSize for U64 {
    type Size = u64;
    fn transform(compressed: [u64; 8]) -> Self {
        let mut test: [u8; 64] = [0; 64];
        let mut i = 0;
        for value in compressed {
            for byte in value.to_be_bytes() {
                test[i] = byte;
            }
        }
    //    let test: [u8; 64] = compressed.iter()
    //         .map(|s| s.as_bytes())
    //         .flat_map(|bytes| bytes)
    //         .map(|f| *f)
    //         .try_into()
    //         .unwrap();
            // .try_into()
            // .unwrap();
            // .try_into()
            // .unwrap();
        U64(test)
    }
}
impl Extract for U64 {
    type Size = u8;
    const N: usize = 64;
    fn take(self) -> [Self::Size; Self::N] {
        todo!()
    }

    // fn take::<64>(self) -> [u8; 64] {
    //     self.0
    // }
}