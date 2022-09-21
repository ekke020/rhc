enum Sha2_512Base {
    SHA384,
    SHA512,
    Sha512_224,
    Sha512_256,
}

impl Sha2_512Base {
    fn get_constants(self) -> &'static [u64; 8] {
        match self {
            Sha2_512Base::SHA384 => &[
                0xcbbb9d5dc1059ed8,
                0x629a292a367cd507,
                0x9159015a3070dd17,
                0x152fecd8f70e5939,
                0x67332667ffc00b31,
                0x8eb44a8768581511,
                0xdb0c2e0d64f98fa7,
                0x47b5481dbefa4fa4,
            ],
            Sha2_512Base::SHA512 => &[
                0x6a09e667f3bcc908,
                0xbb67ae8584caa73b,
                0x3c6ef372fe94f82b,
                0xa54ff53a5f1d36f1,
                0x510e527fade682d1,
                0x9b05688c2b3e6c1f,
                0x1f83d9abfb41bd6b,
                0x5be0cd19137e2179,
            ],
            Sha2_512Base::Sha512_224 => todo!(),
            Sha2_512Base::Sha512_256 => todo!(),
        }
    }
}
