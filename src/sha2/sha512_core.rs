use crate::sha2::bit_utils::u64_addition;

use super::bit_utils::{right_shift, u64_rotate};
use super::consts::{State512, H512_384, H512_512, K64, H256_256, State256, H512_256};
use super::implementation::{CompressionSize, Hash, Sha, U48, U64, U32};
pub struct Sha512 {
    value: Vec<u8>, // The value that was provided.
    state: State512,
    compressed: Option<U64>, // The final hash from the value.
}

impl Sha for Sha512 {
    fn new(value: Vec<u8>) -> Self {
        Self {
            value,
            state: H512_512,
            compressed: None,
        }
    }
}

impl Hash<U64> for Sha512 {
    fn reload(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    fn run(&mut self) {
        let mut buffer = self.state;
        for chunk in self.value.chunks_mut(128) {
            let message = mutate_chunk(chunk);
            buffer = compression(message, buffer);
        }
        self.compressed = Some(U64::transform(buffer));
    }

    fn extract(&mut self) -> U64 {
        self.compressed
            .take()
            .expect("Can't extract before running hash")
    }
}

pub struct Sha384 {
    value: Vec<u8>, // The value that was provided.
    state: State512,
    compressed: Option<U48>, // The final hash from the value.
}

impl Sha for Sha384 {
    fn new(value: Vec<u8>) -> Self {
        Self {
            value,
            state: H512_384,
            compressed: None,
        }
    }
}

impl Hash<U48> for Sha384 {
    fn reload(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    fn run(&mut self) {
        let mut buffer = self.state;
        for chunk in self.value.chunks_mut(128) {
            let message = mutate_chunk(chunk);
            buffer = compression(message, buffer);
        }
        self.compressed = Some(U48::transform(buffer));
    }

    fn extract(&mut self) -> U48 {
        self.compressed
            .take()
            .expect("Can't extract before running hash")
    }
}

pub struct Sha512_256 {
    value: Vec<u8>, // The value that was provided.
    state: State512,
    compressed: Option<U64>, // The final hash from the value.
}

impl Sha for Sha512_256 {
    fn new(value: Vec<u8>) -> Self {
        Self {
            value,
            state: H512_256,
            compressed: None,
        }
    }
}

impl Hash<U64> for Sha512_256 {
    fn reload(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    fn run(&mut self) {
        let mut buffer = self.state;
        for chunk in self.value.chunks_mut(128) {
            let message = mutate_chunk(chunk);
            buffer = compression(message, buffer);
        }
        self.compressed = Some(U64::transform(buffer));
    }

    fn extract(&mut self) -> U64 {
        self.compressed
            .take()
            .expect("Can't extract before running hash")
    }
}

fn mutate_chunk(message: &[u8]) -> [u64; 80] {
    let mut message_schedule: [u64; 80] = [0; 80];
    let mut i = 0;

    // Concatenate 8 bytes into a 64bit word.
    message.windows(8).step_by(8).for_each(|bytes| {
        message_schedule[i] = concatenate_bytes(bytes);
        i += 1;
    });
    // Manipulate the bits after index 15.
    for i in 16..80 {
        let s0: u64 = bit_manipulation(&message_schedule[i - 15], 1, 8, 7);
        let s1: u64 = bit_manipulation(&message_schedule[i - 2], 19, 61, 6);
        let [s2, s3] = [message_schedule[i - 16], message_schedule[i - 7]];
        message_schedule[i] = u64_addition!(s0, s1, s2, s3);
    }
    message_schedule
}

fn concatenate_bytes(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .map(|byte| *byte as u64)
        .reduce(|con, byte| (con << 8) | byte)
        .expect("Concatenation can't be performed on an empty array.")
}

fn bit_manipulation(word_64_bit: &u64, r1: u8, r2: u8, r3: u8) -> u64 {
    let n0 = u64_rotate(word_64_bit, r1);
    let n1 = u64_rotate(word_64_bit, r2);
    let n2 = right_shift!(word_64_bit, r3);
    n0 ^ n1 ^ n2
}

fn compression(message: [u64; 80], compressed: [u64; 8]) -> [u64; 8] {
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = compressed;

    for i in 0..80 {
        let s1 = u64_rotate(&e, 14) ^ u64_rotate(&e, 18) ^ u64_rotate(&e, 41);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = u64_addition!(h, s1, ch, K64[i], message[i]);
        let s0 = u64_rotate(&a, 28) ^ u64_rotate(&a, 34) ^ u64_rotate(&a, 39);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = u64_addition!(s0, maj);
        h = g;
        g = f;
        f = e;
        e = u64_addition!(d, temp1);
        d = c;
        c = b;
        b = a;
        a = u64_addition!(temp1, temp2);
        println!("{}: {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x}",i ,a ,b ,c ,d ,e ,f ,g ,h);
    }
    let compressed: [u64; 8] = [
        u64_addition!(compressed[0], a),
        u64_addition!(compressed[1], b),
        u64_addition!(compressed[2], c),
        u64_addition!(compressed[3], d),
        u64_addition!(compressed[4], e),
        u64_addition!(compressed[5], f),
        u64_addition!(compressed[6], g),
        u64_addition!(compressed[7], h),
    ];
    compressed
}

#[test]
fn test_concatenate_bytes() {
    let bytes = "test".as_bytes();
    assert_eq!(concatenate_bytes(bytes), 0x74657374);
    assert_ne!(concatenate_bytes(bytes), 0x74657375);
}

#[test]
fn test_bit_manipulation() {
    assert_eq!(bit_manipulation(&0xbda07892e4b75bc, 1, 8, 7), 0xb9f16dcc0c576740);
    assert_eq!(bit_manipulation(&0xba1852fbff133824, 19, 61, 6), 0xb52e61d71d3af227);
}

#[test]
fn test_mutate_chunk() {
    // pad of "test"
    let padded_value: Vec<u8> = vec![
        0x74, 0x65, 0x73, 0x74, 0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x20,
    ];
    let correct: [u64; 80] = [
        0x7465737480000000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 
        0x20, 0x7465737480000000, 0x4000000000100, 0xa2fa00e57c6e9003, 0x100080000804, 0xc75b9b77369995d0,
        0x100004400005000, 0xb7bad3f1b782e22, 0x76697575900a0140, 0x5e38c623520b73c7, 0x95b40199f52d200a,
        0x9ec649c585d7e1b8, 0xd35329f685282041, 0x97e4517f19132f8, 0xa957e6b8b1e3a04c, 0x3f245f9abf72e89,
        0xd4fadbc8c0990ebf, 0x46847b366d6be19, 0xf7b727ef943ee840, 0x6eddef1536ef7b48, 0x1fd273446f02e673,
        0x973852112c0207f, 0xceb550baa04cd3cd, 0xabda905f95b47ca5, 0xd0b9d0c606c55cf8, 0x6b4428a7b8d309d5,
        0x2865da4bccab56bd, 0xa32c17832b662601, 0x2c42f9849e3cc11, 0xd0afd96cf86ec984, 0xcdd7f76fd26a91ac,
        0x87b9675f77841fda, 0x1b351c9bb31d1acf, 0x23bffc311b3a4c49, 0x955ea4f7b9896eda, 0xc363a7f55e3c638b,
        0x719fb25717fca4b4, 0x96b6bd96d556b4d4, 0xe12d41fb58260c4c, 0xb9b273a3b88d9fea, 0x2000cc19db102c83,
        0xa67514cfb1578d8f, 0xc2530a38e224937b, 0x65b1ac76e66f5172, 0xa9f6fb27c656e54, 0x43d088c78746b74f,
        0x18fa6498ac7f5df8, 0xb3d4f146fd2046f4, 0xc6753dd090fd1886, 0xbda07892e4b75bc, 0xfaa5ab3eb30657f8,
        0x8aa966d1637bcf95, 0x7089903a48b16eb2, 0x1bfa1bb4965b3efb, 0xf46f0c0b2c84a5d4, 0x4be86f823565e88c,
        0xbdbe2be23aea3a30, 0x200784e34a9c9efb, 0x35b425e3c883ba8a, 0xbaeeb0244129c5ad, 0xeeec7aa0ed6ee178, 
        0x19bf5e7401e48e9b, 0xba1852fbff133824, 0xb2528cc5a6c7736c, 0x559c9257052c10e8
    ];
    let mutated = mutate_chunk(&padded_value);
    assert_eq!(mutated, correct);
}

#[test]
fn test_compression() {
    // mutated string of "test"
    let chunk: [u64; 80] = [
        0x7465737480000000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 
        0x20, 0x7465737480000000, 0x4000000000100, 0xa2fa00e57c6e9003, 0x100080000804, 0xc75b9b77369995d0,
        0x100004400005000, 0xb7bad3f1b782e22, 0x76697575900a0140, 0x5e38c623520b73c7, 0x95b40199f52d200a,
        0x9ec649c585d7e1b8, 0xd35329f685282041, 0x97e4517f19132f8, 0xa957e6b8b1e3a04c, 0x3f245f9abf72e89,
        0xd4fadbc8c0990ebf, 0x46847b366d6be19, 0xf7b727ef943ee840, 0x6eddef1536ef7b48, 0x1fd273446f02e673,
        0x973852112c0207f, 0xceb550baa04cd3cd, 0xabda905f95b47ca5, 0xd0b9d0c606c55cf8, 0x6b4428a7b8d309d5,
        0x2865da4bccab56bd, 0xa32c17832b662601, 0x2c42f9849e3cc11, 0xd0afd96cf86ec984, 0xcdd7f76fd26a91ac,
        0x87b9675f77841fda, 0x1b351c9bb31d1acf, 0x23bffc311b3a4c49, 0x955ea4f7b9896eda, 0xc363a7f55e3c638b,
        0x719fb25717fca4b4, 0x96b6bd96d556b4d4, 0xe12d41fb58260c4c, 0xb9b273a3b88d9fea, 0x2000cc19db102c83,
        0xa67514cfb1578d8f, 0xc2530a38e224937b, 0x65b1ac76e66f5172, 0xa9f6fb27c656e54, 0x43d088c78746b74f,
        0x18fa6498ac7f5df8, 0xb3d4f146fd2046f4, 0xc6753dd090fd1886, 0xbda07892e4b75bc, 0xfaa5ab3eb30657f8,
        0x8aa966d1637bcf95, 0x7089903a48b16eb2, 0x1bfa1bb4965b3efb, 0xf46f0c0b2c84a5d4, 0x4be86f823565e88c,
        0xbdbe2be23aea3a30, 0x200784e34a9c9efb, 0x35b425e3c883ba8a, 0xbaeeb0244129c5ad, 0xeeec7aa0ed6ee178, 
        0x19bf5e7401e48e9b, 0xba1852fbff133824, 0xb2528cc5a6c7736c, 0x559c9257052c10e8
    ];
    // correct result of compression
    let correct: [u64; 8] = [
        0x9f86d081, 0x884c7d65, 0x9a2feaa0, 0xc55ad015, 0xa3bf4f1b, 0x2b0b822c, 0xd15d6c15,
        0xb0f00a08,
    ];
    assert_eq!(compression(chunk, H512_512), correct);
}
