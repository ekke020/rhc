/// Lazily initializes a vector with 0x00.
///
/// ## Arguments
///
/// * `length` - The total length of the byte array.
/// * `size` - The base multiple target.
///
/// ## About
/// The sha implementation requires that each byte is initialized to 0x00.
/// It takes a length which should be the total length of the supplied strings
/// byte array, and also a size which is the targeted chunk size, for example:
/// Sha256 based hashes will supply a chunk size of 64 since the targeted chunk
/// size is 512 bits, and this macro will evaluate the total length of the string
/// and then calculate the multiple of the supplied size value.
///
/// ## Why
/// This macro is required because the total size of the vector can't be known
/// at compile time.
///
macro_rules! lazy_vector {
    ($length:expr, $size:expr) => {{
        let m: usize = ($length + 8) / $size;
        let capacity: usize = ((m + 1) * ($size)) - 8;
        let mut temp: Vec<u8> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            temp.push(0x00);
        }
        temp
    }};
}
/// Performs u32 addition with overflow
///
/// ## Arguments
///
/// * `x` - The first u32.
/// * `y` - The second u32.
/// * `variadic` - Can accept any number of arguments greater than two.
///
/// ## About
/// Sha256 & sha224 calculates addition using the formula (modulo 2³²)
///
/// ## Why
/// This macro enables the restricted addition required by the sha 224 & 256 hashes.
/// It also works as a variadic function for better readability.
///
macro_rules! u32_addition {
    ($x:expr, $y:expr) => {
        (($x as u64 + $y as u64) % (u32::MAX as u64 + 1)) as u32
    };
    ($x:expr $( , $y:expr)*) => {
        (($x as u64 + u32_addition!($($y),*) as u64) % (u32::MAX as u64 + 1)) as u32
    };

}
/// Performs u64 addition with overflow
///
/// ## Arguments
///
/// * `x` - The first u64.
/// * `y` - The second u64.
/// * `variadic` - Can accept any number of arguments greater than two.
///
/// ## About
/// Sha512, 384, 512_224 & 512_256 calculates addition using the formula (modulo 2⁶⁴)
///
/// ## Why
/// This macro enables the restricted addition required by the sha 512 hashes.
/// It also works as a variadic function for better readability.
///
macro_rules! u64_addition {
    ($x:expr, $y:expr) => {
        (($x as u128 + $y as u128) % (u64::MAX as u128 + 1)) as u64
    };
    ($x:expr $( , $y:expr)*) => {
        (($x as u128 + u64_addition!($($y),*) as u128) % (u64::MAX as u128 + 1)) as u64
    };
}

pub(super) use {lazy_vector, u32_addition, u64_addition};

pub(super) fn pad<const N: usize>(bytes: &[u8]) -> Vec<u8> {
    let mut decimal = lazy_vector!(bytes.len(), N);
    // Add the binary values to the array.
    bytes
        .iter()
        .enumerate()
        .for_each(|(i, byte)| decimal[i] = *byte);

    // Append a single bit after the last binary.
    decimal[bytes.len()] = 0x80;
    // Get the big endian representation of the length of value.
    let big_endian_rep = (bytes.len() * 8).to_be_bytes();
    big_endian_rep.iter().for_each(|byte| decimal.push(*byte));
    decimal
}

pub fn to_bytes_32<const N: usize>(buffer: &[u32]) -> [u8; N] {
    buffer
        .iter()
        .flat_map(|v| v.to_be_bytes())
        .take(N)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap_or_else(|err: Vec<u8>| panic!("N was {N} when it should not exceed {}", err.len()))
}

pub fn to_bytes_64<const N: usize>(buffer: &[u64]) -> [u8; N] {
    buffer
        .iter()
        .flat_map(|v| v.to_be_bytes())
        .take(N)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap_or_else(|err: Vec<u8>| panic!("N was {N} when it should not exceed {}", err.len()))
}

pub(super) const RUN_ERROR: &str = "Load with value before running the algorithm";
pub(super) const EXTRACT_ERROR: &str = "Can't extract before running hash";

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn to_bytes_32_should_return_correct_bytes() {
        let buffer = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 
            0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
        let result = to_bytes_32::<32>(&buffer);
        assert_eq!(result, [106, 9, 230, 103, 187, 103, 174, 133, 60, 110, 243, 114, 165, 79, 245, 58, 81, 14, 82, 127, 155, 5, 104, 140, 31, 131, 217, 171, 91, 224, 205, 25]);
    }
    
    #[test]
    #[should_panic]
    fn to_bytes_32_should_panic_if_n_is_to_large() {
        to_bytes_32::<48>(&[0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,]);
    }

    #[test]
    fn to_bytes_64_should_return_correct_bytes() {
        let buffer = [
            0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
            0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179];
        let result = to_bytes_64::<64>(&buffer);
        assert_eq!(result, [
            106, 9, 230, 103, 243, 188, 201, 8, 187, 103, 174, 133, 132, 202, 167, 59, 60, 110,
            243, 114, 254, 148, 248, 43, 165, 79, 245, 58, 95, 29, 54, 241, 81, 14, 82, 127,
            173, 230, 130, 209, 155, 5, 104, 140, 43, 62, 108, 31, 31, 131, 217, 171, 251, 65,
            189, 107, 91, 224, 205, 25, 19, 126, 33, 121
        ]);
    }
    #[test]
    #[should_panic]
    fn to_bytes_64_should_panic_if_n_is_to_large() {
        to_bytes_64::<128>(&[0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,]);
    }

    #[test]
    fn u32_addition_is_correct() {
        assert_eq!(u32_addition!(0x74657374, 0x676F6F64), 0xDBD4E2D8);
        assert_ne!(u32_addition!(0x7361640A, 0x6C75636B), 0xDBD4E2D8);
    }
    
    #[test]
    fn u64_addition_is_correct() {
        assert_eq!(u64_addition!(0x5468697349734154657374466F7255, 0x36344164646974696F6E), 0x95c9d7ddbad8e1c3);
        assert_ne!(u64_addition!(0x7361640A, 0x6C75636B), 0xDBD4E2D8);
    }

    #[test]
    fn pad_appends_single_bit_at_end() {
        let test = "test";
        let k = pad::<64>(test.as_bytes());
        assert_eq!(k[4], 128);
    }

    #[test]
    fn pad_appends_length_as_big_endian() {
        let test = "test";
        let k = pad::<128>(test.as_bytes());
        assert_eq!(k[127], 32);
    }

    #[test]
    fn lazy_vector_produces_desired_length() {
        let result = lazy_vector!(4, 512);
        assert_eq!(result.len(), 504);
    }
}
