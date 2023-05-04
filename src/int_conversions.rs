//! Implementations of conversions from different integers.

use super::{
    Bls12381Base, Bls12381Scalar, Curve25519Base, Curve25519Scalar, PallasBase, PallasScalar,
};

/// Implements `From<T>`, assuming that `Self.0` is convertable using `as`.
macro_rules! from_impls {
    ($t:ty, $builtin:ty, $($integer:ident)*) => ($(
        impl From<$integer> for $t {
            fn from(_value: $integer) -> Self {
                // Self(value as $builtin)
                todo!("field types casts are not yet implemented");
            }
        }
    )*)
}

from_impls!(
    Bls12381Base, __zkllvm_field_bls12381_base, bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128
);

from_impls!(
    Bls12381Scalar, __zkllvm_field_bls12381_base, bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128
);

from_impls!(
    Curve25519Base, __zkllvm_field_bls12381_base, bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128
);

from_impls!(
    Curve25519Scalar, __zkllvm_field_bls12381_base, bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128
);

from_impls!(
    PallasBase, __zkllvm_field_bls12381_base, bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128
);

from_impls!(
    PallasScalar, __zkllvm_field_bls12381_base, bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128
);
