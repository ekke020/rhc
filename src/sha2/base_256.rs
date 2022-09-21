pub enum Sha2_256Base {
    SHA224,
    Sha256,
}

impl Sha2_256Base {
    fn get_constants(self) -> &'static [u32; 8] {
        match self {
            Sha2_256Base::SHA224 => &[
                0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7,
                0xbefa4fa4,
            ],
            Sha2_256Base::Sha256 => &[
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
        }
    }

    fn get_length(self) -> usize {
        match self {
            Sha2_256Base::SHA224 => 7,
            Sha2_256Base::Sha256 => 8,
        }
    }
}