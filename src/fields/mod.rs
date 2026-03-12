mod fp;
mod fq12;
mod fq2;
mod fq6;

use crate::arith::U256;
use core::{
    fmt::Debug,
    ops::{Add, Mul, Neg, Sub},
};

pub use self::fp::{Fq, Fr, const_fq};
pub use self::fq2::{Fq2, fq2_nonresidue};
pub use self::fq6::Fq6;
pub use self::fq12::Fq12;

pub trait FieldElement:
    Sized
    + Copy
    + Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + PartialEq
    + Eq
    + Debug
{
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(&self) -> bool;
    fn squared(&self) -> Self {
        (*self) * (*self)
    }
    fn inverse(self) -> Option<Self>;
    fn pow<I: Into<U256>>(&self, by: I) -> Self {
        let mut res = Self::one();

        for i in by.into().bits() {
            res = res.squared();
            if i {
                res = *self * res;
            }
        }

        res
    }
}
