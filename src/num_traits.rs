//! Implementations of some of [`num-traits`](https://docs.rs/num-traits/) traits.
//!
//! Currently implemented:
//! * [`Zero`]
//! * [`One`]

use num_traits::{One, Zero};

use super::{
    Bls12381Base, Bls12381Scalar, Curve25519Base, Curve25519Scalar, PallasBase, PallasScalar,
};

/// Implements [`One`] and [`Zero`].
macro_rules! num_traits_impl {
    ($($t:ty)*) => ($(
        impl Zero for $t {
            #[inline]
            fn zero() -> Self {
                Self(0g)
            }

            #[inline]
            fn is_zero(&self) -> bool {
                self == &Self::zero()
            }
        }

        impl One for $t {
            fn one() -> Self {
                Self(1g)
            }
        }
    )*)
}

num_traits_impl!(
    Bls12381Base
    Bls12381Scalar
    Curve25519Base
    Curve25519Scalar
    PallasBase
    PallasScalar
);
