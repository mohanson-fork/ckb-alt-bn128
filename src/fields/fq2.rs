use crate::arith::{U256, U512};
use crate::fields::fp::{FQ, FQ_MINUS1_DIV2, FQ_MINUS3_DIV4};
use crate::fields::{FieldElement, Fq};
use core::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Fq2 {
    c0: Fq,
    c1: Fq,
}

impl Fq2 {
    pub fn new(c0: Fq, c1: Fq) -> Self {
        Fq2 { c0: c0, c1: c1 }
    }

    pub fn scale(&self, by: Fq) -> Self {
        Fq2 {
            c0: self.c0 * by,
            c1: self.c1 * by,
        }
    }

    pub fn mul_by_nonresidue(&self) -> Self {
        // Do *self * fq2_nonresidue()
        // The fq2_nonresidue() = 9 + u  (where u^2 = fq_non_residue in Fq)
        // (a + b*u) * (9 + u) = (9a - b) + (a + 9b)*u
        // Compute 9*x = 8*x + x using only field additions (no Montgomery mul)
        let nine_c0 = {
            let t = self.c0 + self.c0; // 2
            let t = t + t; // 4
            let t = t + t; // 8
            t + self.c0 // 9
        };
        let nine_c1 = {
            let t = self.c1 + self.c1; // 2
            let t = t + t; // 4
            let t = t + t; // 8
            t + self.c1 // 9
        };
        Fq2 {
            c0: nine_c0 - self.c1,
            c1: self.c0 + nine_c1,
        }
    }

    pub fn frobenius_map(&self, power: usize) -> Self {
        if power % 2 == 0 {
            *self
        } else {
            Fq2 {
                c0: self.c0,
                c1: -self.c1,
            }
        }
    }

    pub fn real(&self) -> &Fq {
        &self.c0
    }

    pub fn imaginary(&self) -> &Fq {
        &self.c1
    }
}

impl FieldElement for Fq2 {
    fn zero() -> Self {
        Fq2 {
            c0: Fq::zero(),
            c1: Fq::zero(),
        }
    }

    fn one() -> Self {
        Fq2 {
            c0: Fq::one(),
            c1: Fq::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero()
    }

    fn squared(&self) -> Self {
        let ab = self.c0 * self.c1;
        Fq2 {
            c0: (self.c0 - self.c1) * (self.c0 + self.c1),
            c1: ab + ab,
        }
    }

    fn inverse(self) -> Option<Self> {
        // "High-Speed Software Implementation of the Optimal Ate Pairing
        // over Barreto–Naehrig Curves"; Algorithm 8

        match (self.c0.squared() + self.c1.squared()).inverse() {
            Some(t) => Some(Fq2 {
                c0: self.c0 * t,
                c1: -(self.c1 * t),
            }),
            None => None,
        }
    }
}

crate::fields::impl_field_assign_ops!(Fq2);

impl Mul for Fq2 {
    type Output = Fq2;

    fn mul(self, other: Fq2) -> Fq2 {
        // Devegili OhEig Scott Dahab
        //     Multiplication and Squaring on Pairing-Friendly Fields.pdf
        //     Section 3 (Karatsuba)

        let aa = self.c0 * other.c0;
        let bb = self.c1 * other.c1;

        Fq2 {
            c0: aa - bb,
            c1: (self.c0 + self.c1) * (other.c0 + other.c1) - aa - bb,
        }
    }
}

impl Sub for Fq2 {
    type Output = Fq2;

    fn sub(self, other: Fq2) -> Fq2 {
        Fq2 {
            c0: self.c0 - other.c0,
            c1: self.c1 - other.c1,
        }
    }
}

impl Add for Fq2 {
    type Output = Fq2;

    fn add(self, other: Fq2) -> Fq2 {
        Fq2 {
            c0: self.c0 + other.c0,
            c1: self.c1 + other.c1,
        }
    }
}

impl Neg for Fq2 {
    type Output = Fq2;

    fn neg(self) -> Fq2 {
        Fq2 {
            c0: -self.c0,
            c1: -self.c1,
        }
    }
}

impl Fq2 {
    pub fn i() -> Fq2 {
        Fq2::new(Fq::zero(), Fq::one())
    }

    pub fn sqrt(&self) -> Option<Self> {
        let a1 = self.pow::<U256>((FQ_MINUS3_DIV4).into());
        let a1a = a1 * *self;
        let alpha = a1 * a1a;
        let a0 = alpha.pow(FQ) * alpha;

        if a0 == Fq2::one().neg() {
            return None;
        }

        if alpha == Fq2::one().neg() {
            Some(Self::i() * a1a)
        } else {
            let b = (alpha + Fq2::one()).pow::<U256>((FQ_MINUS1_DIV2).into());
            Some(b * a1a)
        }
    }

    pub fn to_u512(&self) -> U512 {
        let c0: U256 = (*self.real()).into();
        let c1: U256 = (*self.imaginary()).into();

        U512::new(&c1, &c0, &FQ)
    }
}
