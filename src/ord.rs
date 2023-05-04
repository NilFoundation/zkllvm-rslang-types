//! Order and partial order implementations.

use std::cmp::{Ord, Ordering, PartialOrd};

use super::{
    Bls12381Base, Bls12381Scalar, Curve25519Base, Curve25519Scalar, PallasBase, PallasScalar,
};

/// Implements [`PartialOrd`] and [`Ord`].
macro_rules! ord_impl {
    ($($t:ty)*) => ($(
        impl PartialOrd for $t {
            /// Field elements are not comparable by default.
            ///
            /// So this partial order is trivial and this function always returns `None`.
            fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
                None
            }
        }

        impl Ord for $t {
            /// Field elements are not comparable by default, so this function always panics.
            fn cmp(&self, _: &Self) -> Ordering {
                panic!("fields are not comparable");
            }
        }
    )*)
}

ord_impl!(
    Bls12381Base
    Bls12381Scalar
    Curve25519Base
    Curve25519Scalar
    PallasBase
    PallasScalar
);
