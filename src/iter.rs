//! Implementations of some of [`std::iter`] traits.
//!
//! Currently implemented:
//! * [`Sum`]
//! * [`Product`]

use std::iter::{Iterator, Product, Sum};

use super::{
    Bls12381Base, Bls12381Scalar, Curve25519Base, Curve25519Scalar, PallasBase, PallasScalar,
};

/// Implements [`Sum`] and [`Product`].
macro_rules! sum_product_impl {
    ($($t:ty)*) => ($(
        impl Sum for $t {
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    Self(0g),
                    |a, b| a + b,
                )
            }
        }

        impl Product for $t {
            fn product<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    Self(1g),
                    |a, b| a * b,
                )
            }
        }

        impl<'a> Sum<&'a $t> for $t {
            fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    Self(0g),
                    |a, b| a + b,
                )
            }
        }

        impl<'a> Product<&'a $t> for $t {
            fn product<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    Self(1g),
                    |a, b| a * b,
                )
            }
        }
    )*)
}

sum_product_impl!(
    Bls12381Base
    Bls12381Scalar
    Curve25519Base
    Curve25519Scalar
    PallasBase
    PallasScalar
);
