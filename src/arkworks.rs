//! Implementations designed specifically for [`arkworks`](https://arkworks.rs/).

use std::iter::Once;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use ark_ff::{Field, Fp256, MontBackend, MontConfig};
use ark_ff::fields::{LegendreSymbol, SqrtPrecomputation};
use ark_serialize::{
    CanonicalSerialize, CanonicalSerializeWithFlags, CanonicalDeserialize,
    CanonicalDeserializeWithFlags, Compress, Flags, SerializationError, Valid, Validate,
};
use ark_std::UniformRand;
use ark_std::rand::Rng;
use ark_std::io::{Write, Read};

use super::{PallasBase, PallasScalar};

/// Implements `op<&mut Self>` for all arithmetic operations.
///
/// These implementations are kind of specific for arkworks needs afaik, so they are placed here.
macro_rules! mut_arith_impl {
    ($($t:ty)*) => ($(
        impl<'a> Add<&'a mut Self> for $t {
            type Output = $t;

            #[inline]
            fn add(self, other: &'a mut $t) -> $t {
                Self(self.0.add(other.0))
            }
        }

        impl<'a> Sub<&'a mut Self> for $t {
            type Output = $t;

            #[inline]
            fn sub(self, other: &'a mut $t) -> $t {
                Self(self.0.sub(other.0))
            }
        }

        impl<'a> Mul<&'a mut Self> for $t {
            type Output = $t;

            #[inline]
            fn mul(self, other: &'a mut $t) -> $t {
                Self(self.0.mul(other.0))
            }
        }

        impl<'a> Div<&'a mut Self> for $t {
            type Output = $t;

            #[inline]
            fn div(self, other: &'a mut $t) -> $t {
                Self(self.0.div(other.0))
            }
        }

        impl<'a> Rem<&'a mut Self> for $t {
            type Output = $t;

            #[inline]
            fn rem(self, other: &'a mut $t) -> $t {
                Self(self.0.rem(other.0))
            }
        }

        impl<'a> AddAssign<&'a mut Self> for $t {
            #[inline]
            fn add_assign(&mut self, other: &'a mut $t) {
                self.0.add_assign(other.0);
            }
        }

        impl<'a> SubAssign<&'a mut Self> for $t {
            #[inline]
            fn sub_assign(&mut self, other: &'a mut $t) {
                self.0.sub_assign(other.0);
            }
        }

        impl<'a> MulAssign<&'a mut Self> for $t {
            #[inline]
            fn mul_assign(&mut self, other: &'a mut $t) {
                self.0.mul_assign(other.0);
            }
        }

        impl<'a> DivAssign<&'a mut Self> for $t {
            #[inline]
            fn div_assign(&mut self, other: &'a mut $t) {
                self.0.div_assign(other.0);
            }
        }

        impl<'a> RemAssign<&'a mut Self> for $t {
            #[inline]
            fn rem_assign(&mut self, other: &'a mut $t) {
                self.0.rem_assign(other.0);
            }
        }
    )*)
}

mut_arith_impl!(
    PallasBase
    PallasScalar
);

/// Implements [`UniformRand`].
macro_rules! uniform_rand_impl {
    ($($t:ty)*) => ($(
        impl UniformRand for $t {
            fn rand<R: ?Sized>(_: &mut R) -> Self
            where
                R: Rng
            {
                // TODO: actually now I don't know what to do here.
                todo!("uniform rand implementation for fields");
            }
        }
    )*)
}

uniform_rand_impl!(
    PallasBase
    PallasScalar
);

/// Implements a number of serialization traits.
/// We actually do not need them, so they left blank.
macro_rules! serialize_impl {
    ($($t:ty)*) => ($(

        impl CanonicalSerialize for $t {
            fn serialize_with_mode<W: Write>(
                &self,
                _: W,
                _: Compress,
            ) -> Result<(), SerializationError> {
                panic!("serialization is impossible");
            }

            fn serialized_size(&self, _: Compress) -> usize {
                panic!("serialization is impossible");
            }
        }

        impl CanonicalSerializeWithFlags for $t {
            fn serialize_with_flags<W: Write, F: Flags>(
                &self,
                _: W,
                _: F,
            ) -> Result<(), SerializationError> {
                panic!("serialization is impossible");
            }

            fn serialized_size_with_flags<F: Flags>(&self) -> usize {
                panic!("serialization is impossible");
            }
        }

        impl Valid for $t {
            fn check(&self) -> Result<(), SerializationError> {
                panic!("serialization is impossible");
            }
        }

        impl CanonicalDeserialize for $t {
            fn deserialize_with_mode<R: Read>(
                _: R,
                _: Compress,
                _: Validate,
            ) -> Result<Self, SerializationError> {
                panic!("serialization is impossible");
            }
        }

        impl CanonicalDeserializeWithFlags for $t {
            fn deserialize_with_flags<R: Read, F: Flags>(
                _: R,
            ) -> Result<(Self, F), SerializationError> {
                panic!("serialization is impossible");
            }
        }
    )*)
}

serialize_impl!(
    PallasBase
    PallasScalar
);

#[derive(MontConfig)]
#[modulus = "28948022309329048855892746252171976963363056481941560715954676764349967630337"]
#[generator = "5"]
pub struct PallasBaseConfig;

#[derive(MontConfig)]
#[modulus = "28948022309329048855892746252171976963363056481941647379679742748393362948097"]
#[generator = "5"]
pub struct PallasScalarConfig;

/// Implements [`Field`].
macro_rules! ark_field_impl {
    ($($t:ty, $tcfg:ty)*) => ($(
        impl Field for $t {
            type BasePrimeField = Fp256<MontBackend<$tcfg, 4>>;

            type BasePrimeFieldIter = Once<Self::BasePrimeField>;

            const SQRT_PRECOMP: Option<SqrtPrecomputation<Self>> = None;

            const ZERO: Self = Self(0g);

            const ONE: Self = Self(1g);

            fn extension_degree() -> u64 { todo!() }

            fn to_base_prime_field_elements(&self) -> Self::BasePrimeFieldIter { todo!() }

            fn from_base_prime_field_elems(_elems: &[Self::BasePrimeField]) -> Option<Self> { todo!() }

            fn from_base_prime_field(_elem: Self::BasePrimeField) -> Self { todo!() }

            /// Returns `self + self`.
            #[must_use]
            fn double(&self) -> Self {
                self + self
            }

            /// Doubles `self` in place.
            fn double_in_place(&mut self) -> &mut Self {
                *self += *self;
                self
            }

            /// Negates `self` in place.
            fn neg_in_place(&mut self) -> &mut Self {
                *self = -*self;
                self
            }

            /// Attempt to deserialize a field element, splitting the bitflags metadata
            /// according to `F` specification. Returns `None` if the deserialization
            /// fails.
            ///
            /// This function is primarily intended for sampling random field elements
            /// from a hash-function or RNG output.
            fn from_random_bytes_with_flags<F: Flags>(_bytes: &[u8]) -> Option<(Self, F)> { todo!() }

            /// Returns a `LegendreSymbol`, which indicates whether this field element
            /// is  1 : a quadratic residue
            ///  0 : equal to 0
            /// -1 : a quadratic non-residue
            fn legendre(&self) -> LegendreSymbol { todo!() }

            /// Returns `self * self`.
            #[must_use]
            fn square(&self) -> Self {
                self * self
            }

            /// Squares `self` in place.
            fn square_in_place(&mut self) -> &mut Self {
                *self *= *self;
                self
            }

            /// Computes the multiplicative inverse of `self` if `self` is nonzero.
            #[must_use]
            fn inverse(&self) -> Option<Self> {
                if self == &Self::ZERO {
                    None
                } else {
                    // TODO: is this correct?
                    Some(Self::ONE / self)
                }
            }

            /// If `self.inverse().is_none()`, this just returns `None`. Otherwise, it sets
            /// `self` to `self.inverse().unwrap()`.
            fn inverse_in_place(&mut self) -> Option<&mut Self> {
                self.inverse().map(|inv| {
                    *self = inv;
                    self
                })
            }

            /// Sets `self` to `self^s`, where `s = Self::BasePrimeField::MODULUS^power`.
            /// This is also called the Frobenius automorphism.
            fn frobenius_map_in_place(&mut self, _power: usize) { todo!() }
        }
    )*)
}

ark_field_impl!(
    PallasBase, PallasBaseConfig
    PallasScalar, PallasScalarConfig
);
