use std::{convert::TryInto, fmt::{LowerHex, UpperHex}};
use super::consts::{ByteSize, BYTE_SIZE_64, BYTE_SIZE_128};

pub trait CompressionSize {
    // type Size: Sized;
    fn new(compressed: &[u8]) -> Self;
}
pub trait Extract {
    const S: usize;
    fn take<const N: usize>(self) -> [u8; N];
}
pub struct U64([u8; 64]);
impl CompressionSize for U64 {
    fn new(compressed: &[u8]) -> Self {
        let mut data: [u8; 64] = [0; 64];
        let mut i = 0;
        for value in compressed {
            for byte in value.to_be_bytes() {
                data[i] = byte;
                i += 1;
            }
        }
        U64(data)
    }
}
impl Extract for U64 {
    const S: usize = 64;
    fn take<const N: usize>(self) -> [u8; N] {
        self.0[0..N].try_into().unwrap()
    }
}
// pub struct U64_32([u64; 4]);
// impl CompressionSize for U64_32 {
//     type Size = u64;
//     fn transform(compressed: [u64; 8]) -> Self {
//         U64_32(compressed[0..4].try_into().unwrap())
//     }
// }
// impl Extract for U64_32 {
//     type Size = u64;
//     fn take(self) -> Vec<u64> {
//         self.0.into() 
//     }
// }
// pub struct U64_28([u64; 4]);
// impl CompressionSize for U64_28 {
//     type Size = u64;
//     fn transform(compressed: [u64; 8]) -> Self {
//         print!("Bytes: ");
//         compressed[0..4].iter().for_each(|c| print!("{:01$x?} ", c.to_be_bytes(), 2));
//         println!();
//         U64_28(compressed[0..4].try_into().unwrap())
//     }
// }
// impl Extract for U64_28 {
//     type Size = u64;
//     fn take(self) -> Vec<u64> {
//         self.0.into() 
//     }
// }
pub struct U48([u8; 48]);
impl CompressionSize for U48 {
    type Size = u64;
    fn new(compressed: [u64; 8]) -> Self {
        let mut data: [u8; 48] = [0; 48];
        let mut i = 0;
        for value in compressed {
            for byte in value.to_be_bytes() {
                data[i] = byte;
                i += 1;
            }
        }
        U48(data)
    }
}
impl Extract for U48 {
    const S: usize = 48;
    fn take<const N: usize>(self) -> [u8; N] {
        self.0[0..N].try_into().unwrap()
    }
}
pub struct U32([u8; 32]);
impl CompressionSize for U32 {
    type Size = u64;
    fn new(compressed: [u64; 8]) -> Self {
        let mut data: [u8; 32] = [0; 32];
        let mut i = 0;
        for value in compressed {
            for byte in value.to_be_bytes() {
                data[i] = byte;
                i += 1;
            }
        }
        U32(data)
    }
}
impl Extract for U32 {
    const S: usize = 32;
    fn take<const N: usize>(self) -> [u8; N] {
        self.0[0..N].try_into().unwrap()
    }
}
pub struct U28([u8; 28]);
impl CompressionSize for U28 {
    fn new(compressed: [u64; 8]) -> Self {
        let mut data: [u8; 28] = [0; 28];
        let mut i = 0;
        for value in compressed {
            for byte in value.to_be_bytes() {
                data[i] = byte;
                i += 1;
            }
        }
        U28(data)
    }
}
impl Extract for U28 {
    const S: usize = 28;
    fn take<const N: usize>(self) -> [u8; N] {
        self.0[0..N].try_into().unwrap()
    }
}
pub trait Hash<T>
where T: CompressionSize
{
    fn reload(&mut self, value: &[u8]);

    fn run(&mut self);

    fn extract(&mut self) -> T;

}
pub trait Sha {
    fn new(value: &[u8]) -> Self;
}
