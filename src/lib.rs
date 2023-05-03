//! zkLLVM type wrappers.

#![feature(restricted_std)]

use std::fmt;
use std::ops::{Add, Sub, Neg, Div, Mul, AddAssign, SubAssign, MulAssign, DivAssign};

/// Pallas base field type.
///
/// Wrapper for `__zkllvm_field_pallas_base` type.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct PallasBase(pub __zkllvm_field_pallas_base);

impl Add for PallasBase {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0.add(other.0))
    }
}

impl AddAssign for PallasBase {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0);
    }
}

impl fmt::Debug for PallasBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for PallasBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Div for PallasBase {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(self.0.div(other.0))
    }
}

impl DivAssign for PallasBase {
    fn div_assign(&mut self, other: Self) {
        self.0.div_assign(other.0);
    }
}

impl From<__zkllvm_field_pallas_base> for PallasBase {
    fn from(value: __zkllvm_field_pallas_base) -> Self {
        Self(value)
    }
}

impl Mul for PallasBase {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign for PallasBase {
    fn mul_assign(&mut self, other: Self) {
        self.0.mul_assign(other.0);
    }
}

impl Neg for PallasBase {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl Sub for PallasBase {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0.sub(other.0))
    }
}

impl SubAssign for PallasBase {
    fn sub_assign(&mut self, other: Self) {
        self.0.sub_assign(other.0);
    }
}
