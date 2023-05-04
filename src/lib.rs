//! zkLLVM type wrappers.

#![feature(restricted_std)]

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

/// Bls12381 base field type.
///
/// Wrapper for `__zkllvm_field_bls12381_base` type.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Bls12381Base(pub __zkllvm_field_bls12381_base);

/// Bls12381 scalar field type.
///
/// Wrapper for `__zkllvm_field_bls12381_scalar` type.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Bls12381Scalar(pub __zkllvm_field_bls12381_scalar);

/// Curve25519 base field type.
///
/// Wrapper for `__zkllvm_field_curve25519_base` type.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Curve25519Base(pub __zkllvm_field_curve25519_base);

/// Curve25519 scalar field type.
///
/// Wrapper for `__zkllvm_field_curve25519_scalar` type.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Curve25519Scalar(pub __zkllvm_field_curve25519_scalar);

/// Pallas base field type.
///
/// Wrapper for `__zkllvm_field_pallas_base` type.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct PallasBase(pub __zkllvm_field_pallas_base);

/// Pallas scalar field type.
///
/// Wrapper for `__zkllvm_field_pallas_scalar` type.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct PallasScalar(pub __zkllvm_field_pallas_scalar);

#[macro_use]
mod arith_macros;

arith_impls!(
    Bls12381Base
    Bls12381Scalar
    Curve25519Base
    Curve25519Scalar
    PallasBase
    PallasScalar
);

/// Implements `fmt::Debug` and `fmt::Display`,
/// assuming that `T.0` is `fmt::Debug` and `fmt::Display`.
macro_rules! fmt_impls {
    ($($t:ty)*) => ($(
        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    )*)
}

fmt_impls!(
    Bls12381Base
    Bls12381Scalar
    Curve25519Base
    Curve25519Scalar
    PallasBase
    PallasScalar
);

/// Implements `From<T>`, assuming that `Self.0` is `T`.
macro_rules! from_impl {
    ($($t:ty, $builtin:ident)*) => ($(
        impl From<$builtin> for $t {
            fn from(value: $builtin) -> Self {
                Self(value)
            }
        }
    )*)
}

from_impl!(
    Bls12381Base, __zkllvm_field_bls12381_base
    Bls12381Scalar, __zkllvm_field_bls12381_scalar
    Curve25519Base, __zkllvm_field_curve25519_base
    Curve25519Scalar, __zkllvm_field_curve25519_scalar
    PallasBase, __zkllvm_field_pallas_base
    PallasScalar, __zkllvm_field_pallas_scalar 
);

#[cfg(feature = "hash")]
mod hash;

#[cfg(feature = "ord")]
mod ord;

#[cfg(feature = "int-conversions")]
mod int_conversions;

#[cfg(feature = "num-traits")]
mod num_traits;

#[cfg(feature = "iter")]
mod iter;
