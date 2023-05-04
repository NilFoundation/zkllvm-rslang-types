//! [`Zeroize`] implementations.
//!
//! The actual purpose of this trait is ignored here.
//! We don't need any optimizations, just make it zero and we're done.

use zeroize::Zeroize;

use super::{
    Bls12381Base, Bls12381Scalar, Curve25519Base, Curve25519Scalar, PallasBase, PallasScalar,
};

/// Implements [`Zeroize`].
macro_rules! zeroize_impl {
    ($($t:ty)*) => ($(
        impl Zeroize for $t {
            fn zeroize(&mut self) {
                self.0 = 0g;
            }
        }
    )*)
}

zeroize_impl!(
    Bls12381Base
    Bls12381Scalar
    Curve25519Base
    Curve25519Scalar
    PallasBase
    PallasScalar
);
