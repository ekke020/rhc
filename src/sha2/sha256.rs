use vulkano::buffer::BufferContents;

// round constants for sha256
const k: [u32; 64] = [
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

// let test = 9216;
// let m = test / 512;
// println!("{}", (m + 1) * 512);
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
    // decimal_256[decimal_len] = big_endian_rep;
    for (i, byte) in big_endian_rep.as_bytes().iter().enumerate() {
        decimal_256[decimal_len - i] = *byte;
    }
    decimal_256.iter().for_each(|f| println!("{}", f));
    println!("Big endian value: {:?}", big_endian_rep.as_bytes());
    println!("Length: {}", decimal_256.len());

    for chunk in decimal_256.chunks_mut(64) {
        // mutate_chunk(chunk);
    }
}
pub fn mutate_chunk_new(decimals: &[u8]) {
    let mut word_32_bit: [u32; 64] = [0; 64];
    let mut i = 0;
    decimals.windows(4).step_by(4).for_each(|t| {
        word_32_bit[i] = concatenate_bytes(&[t[0] as u32, t[1] as u32, t[2] as u32, t[3] as u32]);
        i += 1;
    });
    word_32_bit
        .windows(2)
        .step_by(2)
        .for_each(|f| println!("{:?} {:?}", f[0], f[1]));
    let n0 = right_rotate(word_32_bit[1], 7);
    let n1 = right_rotate(word_32_bit[1], 18);
    let n2 = right_shift(word_32_bit[1], 3);
    println!("{:0>32b}", n0);
    println!("{:0>32b}", n1);
    println!("{:0>32b}", n2);
    let answer = n0 ^ n1 ^ n2;
    println!("{answer}");
}

fn bit_manipulation(word_32_bit: u32) -> u32 {
    // TODO: Fix this method
    let n0 = right_rotate(word_32_bit, 7);
    let n1 = right_rotate(word_32_bit, 18);
    let n2 = right_shift(word_32_bit, 3);
    println!("{:0>32b}", n0);
    println!("{:0>32b}", n1);
    println!("{:0>32b}", n2);
    let answer = n0 ^ n1 ^ n2;
    println!("{answer}");
    answer
}

fn concatenate_bytes(bytes: &[u32; 4]) -> u32 {
    let mut decimal_rep: u32 = bytes[0].into();
    decimal_rep = (decimal_rep << 8) | bytes[1];
    decimal_rep = (decimal_rep << 8) | bytes[2];
    decimal_rep = (decimal_rep << 8) | bytes[3];
    decimal_rep
}

fn right_rotate(n: u32, d: u8) -> u32 {
    (n >> d) | (n << (32 - d))
}

fn right_shift(n: u32, d: u8) -> u32 { 
    n >> d
}
// fn mutate_chunk(decimals: &mut [u8]) {
//     let mut w: Vec<String> = vec![format!("{:0>32b}", 0); 64];
//     let mut test: [[u8; 4]; 64] = [[0; 4]; 64];
//     let mut i: usize = 0;
//     decimals.windows(4).step_by(4).for_each(|t| {
//         println!("Deciam array: {:?}", t);
//         test[i] = [t[0], t[1], t[2], t[3]];
//         w[i] = format!("{:0>8b}{:0>8b}{:0>8b}{:0>8b}", t[0], t[1], t[2], t[3]);
//         i += 1;
//     });
//     test.windows(2)
//         .step_by(2)
//         .for_each(|f| println!("{:?} {:?}", f[0], f[1]));
//     w.windows(2)
//         .step_by(2)
//         .for_each(|f| println!("{} {}", f[0], f[1]));

//     println!("Result: {:0>8b} {:0>8b} {:0>8b} {:0>8b}", 111, 32, 119, 111);
//     let result = right_rotate([111, 32, 119, 111], 7);
//     println!(
//         "Result: {} {} {} {}",
//         result[0], result[1], result[2], result[3]
//     );
//     println!(
//         "Result: {:0>8b} {:0>8b} {:0>8b} {:0>8b}",
//         result[0], result[1], result[2], result[3]
//     );

//     for i in 16..63 {
//         let s0 = "";
//         let s1 = "";
//         let s2 = w[i - 16].clone();
//         w[i] = format!("{}{}{}", s2, s0, s1);
//     }

//     w.iter_mut().enumerate().skip(16).for_each(|(i, f)| {
//         // s0 := (w[i-15] rightrotate  7) xor (w[i-15] rightrotate 18) xor (w[i-15] rightshift  3)
//         // s1 := (w[i-2] rightrotate 17) xor (w[i-2] rightrotate 19) xor (w[i-2] rightshift 10)
//         // w[i] := w[i-16] + s0 + w[i-7] + s1
//     });
// }

fn left_rotate(n: u8, d: u8) -> u8 {
    (n << d) | (n >> (8 - d))
}

mod test {
    // "abc" in sha256 hash.
    const test: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
}
