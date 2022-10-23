use super::consts::{State512, H512_384, H512_512, K64};
use super::implementation::{Hash, Sha, U64, CompressionSize};
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
        let mut compressed = self.state;
        for chunk in self.value.chunks_mut(128) {
            let word_32_bit = mutate_chunk(chunk);
            // compressed = compression(word_32_bit, compressed);
        }
        // self.compressed = Some(U64::transform(compressed));
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
    compressed: Option<U64>, // The final hash from the value.
}

fn mutate_chunk(decimals: &[u8]) -> [u64; 128] {
    let mut word_64_bit: [u64; 128] = [0; 128];
    let mut i = 0;

    // Concatenate 4 bytes into a 64bit word.
    decimals.windows(4).step_by(4).for_each(|bytes| {
        word_64_bit[i] = concatenate_bytes(bytes);
        i += 1;
    });
    println!("{:?}", word_64_bit);
    // Manipulate the bits after index 15.
    for i in 16..80 {
        let s0: u64 = bit_manipulation(&word_64_bit[i - 15], 7, 18, 3);
        let s1: u64 = bit_manipulation(&word_64_bit[i - 2], 17, 19, 10);
        let [s2, s3] = [word_64_bit[i - 16], word_64_bit[i - 7]];
        word_64_bit[i] = u32_addition!(s0, s1, s2, s3);
    }
    panic!();
    word_64_bit
}

fn concatenate_bytes(bytes: &[u8]) -> u64 {
    let mut decimal_rep: u64 = bytes[0].into();
    decimal_rep = (decimal_rep << 8) | bytes[1] as u64;
    decimal_rep = (decimal_rep << 8) | bytes[2] as u64;
    decimal_rep = (decimal_rep << 8) | bytes[3] as u64;
    decimal_rep
}

// fn bit_manipulation(word_32_bit: &u64, r1: u8, r2: u8, r3: u8) -> u32 {
//     let n0 = right_rotate(word_32_bit, r1);
//     let n1 = right_rotate(word_32_bit, r2);
//     let n2 = right_shift(word_32_bit, r3);
//     n0 ^ n1 ^ n2
// }

// fn compression(mutated: [u64; 128], compressed: [u32; 8]) -> [u32; 8] {
//     let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = compressed;

//     for i in 0..64 {
//         let s1 = right_rotate(&e, 6) ^ right_rotate(&e, 11) ^ right_rotate(&e, 25);
//         let ch = (e & f) ^ ((!e) & g);
//         let temp1 = u32_addition!(h, s1, ch, K32[i], mutated[i]);
//         let s0 = right_rotate(&a, 2) ^ right_rotate(&a, 13) ^ right_rotate(&a, 22);
//         let maj = (a & b) ^ (a & c) ^ (b & c);
//         let temp2 = u32_addition!(s0, maj);
//         h = g;
//         g = f;
//         f = e;
//         e = u32_addition!(d, temp1);
//         d = c;
//         c = b;
//         b = a;
//         a = u32_addition!(temp1, temp2);
//     }
//     let compressed: [u32; 8] = [
//         u32_addition!(compressed[0], a),
//         u32_addition!(compressed[1], b),
//         u32_addition!(compressed[2], c),
//         u32_addition!(compressed[3], d),
//         u32_addition!(compressed[4], e),
//         u32_addition!(compressed[5], f),
//         u32_addition!(compressed[6], g),
//         u32_addition!(compressed[7], h),
//     ];
//     compressed
// }