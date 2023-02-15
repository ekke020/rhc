use crate::core::package::Package;

use super::result::PasswordMatch;

pub trait Crack {
    fn from(package: &Package) -> Self;
    fn run(&mut self) -> Option<PasswordMatch>;
}

pub struct Wrapper<T> {
    crack: T,
}

impl<T> Wrapper<T> where T: Crack {
    pub fn un_pack(package: &Package) -> T {
        T::from(package)
    }

    pub fn run(&mut self) -> Option<PasswordMatch> {
        self.run()
    }
}
