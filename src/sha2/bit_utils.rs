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
/// This macro is required because the total size of the array can't be known
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
pub(crate) use lazy_vector;
const SHA_256_MAX: u64 = 4294967296;
fn addition_with_overflow<T: Sized>(args: &[u32]) -> u32
// where T: Sized
{
    args.iter()
        .map(|arg| *arg as u64)
        .reduce(|total, arg| (total + arg) % SHA_256_MAX)
        .expect("Empty array is not allowed") as u32
}

fn right_rotate(n: &u32, d: u8) -> u32 {
    (n >> d) | (n << (32 - d))
}

fn right_shift(n: &u32, d: u8) -> u32 {
    n >> d
}

