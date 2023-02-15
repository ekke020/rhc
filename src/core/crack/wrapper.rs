use crate::core::package::Package;

use super::result::PasswordMatch;

pub trait Crack {
    fn from(package: Package, index: usize) -> Self;
    fn run(&mut self) -> Option<PasswordMatch>;
}

pub struct Wrapper<T> {
    crack: T,
}

impl<T> Wrapper<T>
where
    T: Crack,
{
    pub fn un_pack(package: Package, index: usize) -> Self {
        Self {
            crack: T::from(package, index),
        }
    }

    pub fn run(&mut self) -> Option<PasswordMatch> {
        self.crack.run()
    }
}
