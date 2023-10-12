//! zkLLVM type wrappers.

use std::fmt;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

/// Bls12381 curve type.
///
/// Wrapper for `__zkllvm_curve_bls12381` type.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Bls12381(pub __zkllvm_curve_bls12381);

/// Bls12381 base field type.
///
/// Wrapper for `__zkllvm_field_bls12381_base` type.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Bls12381Base(pub __zkllvm_field_bls12381_base);

/// Bls12381 scalar field type.
///
/// Wrapper for `__zkllvm_field_bls12381_scalar` type.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Bls12381Scalar(pub __zkllvm_field_bls12381_scalar);

/// Curve25519 curve type.
///
/// Wrapper for `__zkllvm_curve_curve25519` type.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Curve25519(pub __zkllvm_curve_curve25519);

/// Curve25519 base field type.
///
/// Wrapper for `__zkllvm_field_curve25519_base` type.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Curve25519Base(pub __zkllvm_field_curve25519_base);

/// Curve25519 scalar field type.
///
/// Wrapper for `__zkllvm_field_curve25519_scalar` type.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Curve25519Scalar(pub __zkllvm_field_curve25519_scalar);

/// Pallas curve type.
///
/// Wrapper for `__zkllvm_curve_pallas` type.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Pallas(pub __zkllvm_curve_pallas);

/// Pallas base field type.
///
/// Wrapper for `__zkllvm_field_pallas_base` type.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct PallasBase(pub __zkllvm_field_pallas_base);

/// Pallas scalar field type.
///
/// Wrapper for `__zkllvm_field_pallas_scalar` type.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct PallasScalar(pub __zkllvm_field_pallas_scalar);

/// Vesta curve type.
///
/// Wrapper for `__zkllvm_curve_vesta` type.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Vesta(pub __zkllvm_curve_vesta);

/// Vesta base field type.
///
/// Alias for `PallasScalar` type.
pub type VestaBase = PallasScalar;

/// Vesta scalar field type.
///
/// Alias for `PallasBase` type.
pub type VestaScalar = PallasBase;

/// Implements `Deref` and `DerefMut` on `T`.
macro_rules! deref_impl {
    ($($t:ty, $builtin:ident)*) => ($(
        impl Deref for $t {
            type Target = $builtin;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    )*)
}

deref_impl! {
    Bls12381, __zkllvm_curve_bls12381
    Bls12381Base, __zkllvm_field_bls12381_base
    Bls12381Scalar, __zkllvm_field_bls12381_scalar
    Curve25519, __zkllvm_curve_curve25519
    Curve25519Base, __zkllvm_field_curve25519_base
    Curve25519Scalar, __zkllvm_field_curve25519_scalar
    Pallas, __zkllvm_curve_pallas
    PallasBase, __zkllvm_field_pallas_base
    PallasScalar, __zkllvm_field_pallas_scalar
    Vesta, __zkllvm_curve_vesta
}

/// Implements `fmt::Debug` and `fmt::Display`,
/// assuming that `T.0` is `fmt::Debug` and `fmt::Display`.
macro_rules! fmt_impl {
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

fmt_impl! {
    Bls12381
    Bls12381Base
    Bls12381Scalar
    Curve25519
    Curve25519Base
    Curve25519Scalar
    Pallas
    PallasBase
    PallasScalar
    Vesta
}

macro_rules! from_impl {
    ($($t:ty, $builtin:ident)*) => ($(
        impl From<$builtin> for $t {
            #[inline(always)]
            fn from(value: $builtin) -> Self {
                Self(value)
            }
        }

        impl From<$t> for $builtin {
            #[inline(always)]
            fn from(value: $t) -> Self {
                value.0
            }
        }
    )*)
}

from_impl! {
    Bls12381, __zkllvm_curve_bls12381
    Bls12381Base, __zkllvm_field_bls12381_base
    Bls12381Scalar, __zkllvm_field_bls12381_scalar
    Curve25519, __zkllvm_curve_curve25519
    Curve25519Base, __zkllvm_field_curve25519_base
    Curve25519Scalar, __zkllvm_field_curve25519_scalar
    Pallas, __zkllvm_curve_pallas
    PallasBase, __zkllvm_field_pallas_base
    PallasScalar, __zkllvm_field_pallas_scalar
    Vesta, __zkllvm_curve_vesta
}

#[macro_use]
mod arith_macros;

arith_impl! {
    Bls12381Base
    Bls12381Scalar
    Curve25519Base
    Curve25519Scalar
    PallasBase
    PallasScalar
}

curve_arith_impl! {
    Bls12381, Bls12381Scalar
    Curve25519, Curve25519Scalar
    Pallas, PallasScalar
    Vesta, VestaScalar
}

macro_rules! curve_init_impl {
    ($($curve:ty, $builtin:ty, $base:ty)*) => ($(
        impl $curve {
            /// Create curve element from base field coordinates.
            #[inline(always)]
            pub unsafe fn from_coordinates(x: $base, y: $base) -> Self {
                Self(<$builtin>::from_coordinates(x.0, y.0))
            }

            /// Create neutral curve element.
            #[inline(always)]
            pub fn zero() -> Self {
                Self(<$builtin>::zero())
            }

            /// Create generator (`one`) curve element.
            #[inline(always)]
            pub fn one() -> Self {
                Self(<$builtin>::one())
            }
        }
    )*)
}

curve_init_impl! {
    Bls12381, __zkllvm_curve_bls12381, Bls12381Base
    Curve25519, __zkllvm_curve_curve25519, Curve25519Base
    Pallas, __zkllvm_curve_pallas, PallasBase
    Vesta, __zkllvm_curve_vesta, VestaBase
}

#[cfg(feature = "hash")]
mod hash;

#[cfg(feature = "int-conversions")]
mod int_conversions;

#[cfg(feature = "num-traits")]
mod num_traits;

#[cfg(feature = "iter")]
mod iter;

#[cfg(feature = "zeroize")]
mod zeroize;

#[cfg(feature = "arkworks")]
mod arkworks;
#[cfg(feature = "arkworks")]
pub use arkworks::{
    Bls12381BaseConfig, Bls12381ScalarConfig, Curve25519BaseConfig, Curve25519ScalarConfig,
    PallasBaseConfig, PallasScalarConfig,
};
