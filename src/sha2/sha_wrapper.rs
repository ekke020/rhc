use crate::sha2::wrapper;
pub struct ShaWrapper<T> {
    sha2: T
}

impl<'a, T> ShaWrapper<T> 
where T: wrapper::Hash<'a, T> + wrapper::CompressionSize<u32, 8_usize>
{

}