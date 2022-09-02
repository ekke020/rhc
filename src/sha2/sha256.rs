use vulkano::buffer::BufferContents;

// round constants for sha256
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// Sha256 constants representing the first 32 bits of the square root of the first 8 primes.
const H0: u32 = 0x6a09e667;
const H1: u32 = 0xbb67ae85;
const H2: u32 = 0x3c6ef372;
const H3: u32 = 0xa54ff53a;
const H4: u32 = 0x510e527f;
const H5: u32 = 0x9b05688c;
const H6: u32 = 0x1f83d9ab;
const H7: u32 = 0x5be0cd19;

const ADDITION_OVERFLOW: u64 = 4294967296;

macro_rules! lazy_array {
    ($length:expr, $size:expr) => {{
        let m: usize = $length / $size;
        let capacity: usize = (m + 1) * $size;
        let mut temp: Vec<u8> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            temp.push(0x00);
        }
        temp
    }};
}

pub fn test(value: &str) {
    let bytes = value.as_bytes();
    let mut decimal_256 = lazy_array!(bytes.len(), 64);
    let decimal_len = decimal_256.len() - 1;

    // Add the binary values to the array.
    bytes
        .iter()
        .enumerate()
        .for_each(|(i, byte)| decimal_256[i] = *byte);

    // Append a single bit after the last binary.
    decimal_256[bytes.len()] = 0x80;

    // Get the big endian representation of the length of value.
    let big_endian_rep = bytes.len() * 8;
    for (i, byte) in big_endian_rep.as_bytes().iter().enumerate() {
        decimal_256[decimal_len - i] = *byte;
    }

    // Mutate each 512 bit chunk.
    for chunk in decimal_256.chunks_mut(64) {
        mutate_chunk(chunk);
    }
}

fn mutate_chunk(decimals: &[u8]) {
    let mut word_32_bit: [u32; 64] = [0; 64];
    let mut i = 0;

    // Concatenate 4 bytes into a 32bit word.
    decimals.windows(4).step_by(4).for_each(|bytes| {
        word_32_bit[i] = concatenate_bytes(bytes);
        i += 1;
    });

    // Manipulate the bits after index 15.
    for i in 16..64 {
        let s0: u32 = bit_manipulation(&word_32_bit[i - 15], 7, 18, 3);
        let s1: u32 = bit_manipulation(&word_32_bit[i - 2], 17, 19, 10);
        let t: u32 = addition_with_overflow(s0, s1);
        let t1: u32 = addition_with_overflow(word_32_bit[i - 16], word_32_bit[i - 7]);
        word_32_bit[i] = addition_with_overflow(t, t1);
    }

    word_32_bit
        .windows(2)
        .step_by(2)
        .for_each(|f| println!("{:0>32b} {:0>32b}", f[0], f[1]));

    compression(word_32_bit);
}

fn compression(mutated: [u32; 64]) {
    // TODO: Refactor this.
    let mut a: u32 = H0;
    let mut b: u32 = H1;
    let mut c: u32 = H2;
    let mut d: u32 = H3;
    let mut e: u32 = H4;
    let mut f: u32 = H5;
    let mut g: u32 = H6;
    let mut h: u32 = H7;

    for i in 0..64 {
        let s1 = right_rotate(&e, 6) ^ right_rotate(&e, 11) ^ right_rotate(&e, 25);
        let ch = (e & f) ^ ((!e) & g);
        let mut temp1 = addition_with_overflow(h, s1);
        temp1 = addition_with_overflow(temp1, ch);
        temp1 = addition_with_overflow(temp1, K[i]);
        temp1 = addition_with_overflow(temp1, mutated[i]);
        let s0 = right_rotate(&a, 2) ^ right_rotate(&a, 13) ^ right_rotate(&a, 22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = addition_with_overflow(s0, maj);
        h = g;
        g = f;
        f = e;
        e = addition_with_overflow(d, temp1);
        d = c;
        c = b;
        b = a;
        a = addition_with_overflow(temp1, temp2);
    }
    let v0 = addition_with_overflow(H0, a);
    let v1 = addition_with_overflow(H1, b);
    let v2 = addition_with_overflow(H2, c);
    let v3 = addition_with_overflow(H3, d);
    let v4 = addition_with_overflow(H4, e);
    let v5 = addition_with_overflow(H5, f);
    let v6 = addition_with_overflow(H6, g);
    let v7 = addition_with_overflow(H7, h);

    let answer = format!("{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}", v0, v1, v2, v3, v4, v5, v6, v7);
    println!("{answer}");
}

fn addition_with_overflow(y: u32, x: u32) -> u32 {
    ((y as u64 + x as u64) % ADDITION_OVERFLOW) as u32
}

fn bit_manipulation(word_32_bit: &u32, r1: u8, r2: u8, r3: u8) -> u32 {
    let n0 = right_rotate(word_32_bit, r1);
    let n1 = right_rotate(word_32_bit, r2);
    let n2 = right_shift(word_32_bit, r3);
    n0 ^ n1 ^ n2
}

fn concatenate_bytes(bytes: &[u8]) -> u32 {
    let mut decimal_rep: u32 = bytes[0].into();
    decimal_rep = (decimal_rep << 8) | bytes[1] as u32;
    decimal_rep = (decimal_rep << 8) | bytes[2] as u32;
    decimal_rep = (decimal_rep << 8) | bytes[3] as u32;
    decimal_rep
}

fn right_rotate(n: &u32, d: u8) -> u32 {
    (n >> d) | (n << (32 - d))
}

fn right_shift(n: &u32, d: u8) -> u32 {
    n >> d
}

fn left_rotate(n: u8, d: u8) -> u8 {
    (n << d) | (n >> (8 - d))
}

mod test {
    use super::*;
    // "abc" in sha256 hash.
    // const ABC: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    #[test]
    fn test_right_rotate() {
        assert_eq!(right_rotate(&0x9B05688C, 5), 0x64D82B44);
        assert_eq!(right_rotate(&0x9B05688C, 10), 0x2326C15A);
        assert_ne!(right_rotate(&0x9B05688C, 2), 0x4464D82B);
    }

    #[test]
    fn test_right_shit() {
        assert_eq!(right_shift(&0x9B05688C, 7), 0x1360AD1);
        assert_eq!(right_shift(&0x9B05688C, 10), 0x26C15A);
        assert_ne!(right_shift(&0x9B05688C, 2), 0x9B05688);
    }
    #[test]
    fn test_concatenate_bytes() {
        let bytes = "test".as_bytes();
        assert_eq!(concatenate_bytes(bytes), 0x74657374);
        assert_ne!(concatenate_bytes(bytes), 0x74657375);
    }

    #[test]
    fn test_bit_manipulation() {
        assert_eq!(bit_manipulation(&0x74657374, 5, 10, 5), 0x7D1D195C);
        assert_eq!(bit_manipulation(&0x74657374, 7, 18, 3), 0xBAB97991);
        assert_ne!(bit_manipulation(&0x74657374, 7, 18, 3), 0xBAB97990);
    }

    #[test]
    fn test_addition_with_overflow() {
        assert_eq!(addition_with_overflow(0x74657374, 0x676F6F64), 0xDBD4E2D8);
        assert_ne!(addition_with_overflow(0x7361640A, 0x6C75636B), 0xDBD4E2D8);
    }
}
