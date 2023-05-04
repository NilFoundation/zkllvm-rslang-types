//! Hash implementations.

use std::hash::{Hash, Hasher};

use super::{
    Bls12381Base, Bls12381Scalar, Curve25519Base, Curve25519Scalar, PallasBase, PallasScalar,
};

/// Implements [`Hash`], assuming that `Self.0` is `Hash`.
macro_rules! hash_impl {
    ($($t:ty)*) => ($(
        impl Hash for $t {
            fn hash<H>(&self, _: &mut H)
            where
                H: Hasher
            {
                todo!("hash impls for zk types");
            }
        }
    )*)
}

hash_impl!(
    Bls12381Base
    Bls12381Scalar
    Curve25519Base
    Curve25519Scalar
    PallasBase
    PallasScalar
);
