//! These macros implements basic operations on type wrappers for zkllvm types.
//! "Entry point" is the [`arith_impls`] macro.

/// Implements the unary operator `op &T`
/// based on `op T` where T is expected to be `Copy`.
///
/// Taken from library source code `library/core/src/internal_macros.rs`.
macro_rules! forward_ref_unop {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl $imp for &$t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    }
}

/// Implements binary operators `&T op U`, `T op &U`, `&T op &U`
/// based on `T op U`, where T and U are expected to be `Copy`.
///
/// Taken from library source code `library/core/src/internal_macros.rs`.
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}

/// Implements `T op= &U`, based on `T op= U` where U is expected to be `Copy`able.
///
/// Taken from library source code `library/core/src/internal_macros.rs`.
macro_rules! forward_ref_op_assign {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
    }
}

/// Implements `Add`, assuming that `T.0` and `U.0` are `Add`.
/// Then calls [`forward_ref_binop`] for `+` operation.
macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl Add for $t {
            type Output = $t;

            #[inline]
            fn add(self, other: $t) -> $t {
                Self(self.0.add(other.0))
            }
        }

        forward_ref_binop! { impl Add, add for $t, $t }
    )*)
}

/// Implements `Sub`, assuming that `T.0` and `U.0` are `Sub`.
/// Then calls [`forward_ref_binop`] for `-` operation.
macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl Sub for $t {
            type Output = $t;

            #[inline]
            fn sub(self, other: $t) -> $t {
                Self(self.0.sub(other.0))
            }
        }

        forward_ref_binop! { impl Sub, sub for $t, $t }
    )*)
}

/// Implements `Mul`, assuming that `T.0` and `U.0` are `Mul`.
/// Then calls [`forward_ref_binop`] for `*` operation.
macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl Mul for $t {
            type Output = $t;

            #[inline]
            fn mul(self, other: $t) -> $t {
                Self(self.0.mul(other.0))
            }
        }

        forward_ref_binop! { impl Mul, mul for $t, $t }
    )*)
}

/// Implements `Mul` for curve types.
/// Then calls [`forward_ref_binop`] for `*` operation.
macro_rules! mul_curve_impl {
    ($($curve:ty, $scalar:ty)*) => ($(
        impl Mul<$scalar> for $curve {
            type Output = $curve;

            #[inline]
            fn mul(self, other: $scalar) -> $curve {
                Self(self.0.mul(other.0))
            }
        }

        forward_ref_binop! { impl Mul, mul for $curve, $scalar }

        impl Mul<$curve> for $scalar {
            type Output = $curve;

            #[inline]
            fn mul(self, other: $curve) -> $curve {
                self.0.mul(other.0).into()
            }
        }

        forward_ref_binop! { impl Mul, mul for $scalar, $curve }
    )*)
}

/// Implements `Div`, assuming that `T.0` and `U.0` are `Div`.
/// Then calls [`forward_ref_binop`] for `/` operation.
macro_rules! div_impl {
    ($($t:ty)*) => ($(
        impl Div for $t {
            type Output = $t;

            #[inline]
            fn div(self, other: $t) -> $t {
                Self(self.0.div(other.0))
            }
        }

        forward_ref_binop! { impl Div, div for $t, $t }
    )*)
}

/// Implements `Div` for curve types.
/// Then calls [`forward_ref_binop`] for `/` operation.
macro_rules! div_curve_impl {
    ($($curve:ty, $scalar:ty)*) => ($(
        impl Div<$scalar> for $curve {
            type Output = $curve;

            #[inline]
            fn div(self, other: $scalar) -> $curve {
                Self(self.0.div(other.0))
            }
        }

        forward_ref_binop! { impl Div, div for $curve, $scalar }
    )*)
}

/// Implements `Rem`, assuming that `T.0` and `U.0` are `Rem`.
/// Then calls [`forward_ref_binop`] for `%` operation.
macro_rules! rem_impl {
    ($($t:ty)*) => ($(
        impl Rem for $t {
            type Output = $t;

            #[inline]
            fn rem(self, other: $t) -> $t {
                Self(self.0.rem(other.0))
            }
        }

        forward_ref_binop! { impl Rem, rem for $t, $t }
    )*)
}

/// Implements `Neg`, assuming that `T.0` is `Neg`.
/// Then calls [`forward_ref_unop`] for `-` operation.
macro_rules! neg_impl {
    ($($t:ty)*) => ($(
        impl Neg for $t {
            type Output = $t;

            #[inline]
            fn neg(self) -> $t {
                Self(self.0.neg())
            }
        }

        forward_ref_unop! { impl Neg, neg for $t }
    )*)
}

/// Implements `AddAssign`, assuming that `T.0` and `U.0` are `Add`.
/// Then calls [`forward_ref_op_assign`] for `+=` operation.
macro_rules! add_assign_impl {
    ($($t:ty)+) => ($(
        impl AddAssign for $t {
            #[inline]
            fn add_assign(&mut self, other: $t) {
                self.0.add_assign(other.0);
            }
        }

        forward_ref_op_assign! { impl AddAssign, add_assign for $t, $t }
    )+)
}

/// Implements `SubAssign`, assuming that `T.0` and `U.0` are `SubAssign`.
/// Then calls [`forward_ref_op_assign`] for `-=` operation.
macro_rules! sub_assign_impl {
    ($($t:ty)+) => ($(
        impl SubAssign for $t {
            #[inline]
            fn sub_assign(&mut self, other: $t) {
                self.0.sub_assign(other.0);
            }
        }

        forward_ref_op_assign! { impl SubAssign, sub_assign for $t, $t }
    )+)
}

/// Implements `MulAssign`, assuming that `T.0` and `U.0` are `MulAssign`.
/// Then calls [`forward_ref_op_assign`] for `*=` operation.
macro_rules! mul_assign_impl {
    ($($t:ty)+) => ($(
        impl MulAssign for $t {
            #[inline]
            fn mul_assign(&mut self, other: $t) {
                self.0.mul_assign(other.0);
            }
        }

        forward_ref_op_assign! { impl MulAssign, mul_assign for $t, $t }
    )+)
}

/// Implements `MulAssign` for curves.
/// Then calls [`forward_ref_op_assign`] for `*=` operation.
macro_rules! mul_assign_curve_impl {
    ($($curve:ty, $scalar:ty)+) => ($(
        impl MulAssign<$scalar> for $curve {
            #[inline]
            fn mul_assign(&mut self, other: $scalar) {
                self.0.mul_assign(other.0);
            }
        }

        forward_ref_op_assign! { impl MulAssign, mul_assign for $curve, $scalar }
    )+)
}

/// Implements `DivAssign`, assuming that `T.0` and `U.0` are `DivAssign`.
/// Then calls [`forward_ref_op_assign`] for `/=` operation.
macro_rules! div_assign_impl {
    ($($t:ty)+) => ($(
        impl DivAssign for $t {
            #[inline]
            fn div_assign(&mut self, other: $t) {
                self.0.div_assign(other.0);
            }
        }

        forward_ref_op_assign! { impl DivAssign, div_assign for $t, $t }
    )+)
}

/// Implements `DivAssign` for curves.
/// Then calls [`forward_ref_op_assign`] for `/=` operation.
macro_rules! div_assign_curve_impl {
    ($($curve:ty, $scalar:ty)+) => ($(
        impl DivAssign<$scalar> for $curve {
            #[inline]
            fn div_assign(&mut self, other: $scalar) {
                self.0.div_assign(other.0);
            }
        }

        forward_ref_op_assign! { impl DivAssign, div_assign for $curve, $scalar }
    )+)
}

/// Implements `RemAssign`, assuming that `T.0` and `U.0` are `RemAssign`.
/// Then calls [`forward_ref_op_assign`] for `%=` operation.
macro_rules! rem_assign_impl {
    ($($t:ty)+) => ($(
        impl RemAssign for $t {
            #[inline]
            fn rem_assign(&mut self, other: $t) {
                self.0.rem_assign(other.0);
            }
        }

        forward_ref_op_assign! { impl RemAssign, rem_assign for $t, $t }
    )+)
}

/// All arithmetic impls for given field type.
macro_rules! arith_impl {
    ($($t:ty)*) => ($(
        add_impl! { $t }
        sub_impl! { $t }
        mul_impl! { $t }
        div_impl! { $t }
        rem_impl! { $t }
        neg_impl! { $t }
        add_assign_impl! { $t }
        sub_assign_impl! { $t }
        mul_assign_impl! { $t }
        div_assign_impl! { $t }
        rem_assign_impl! { $t }
    )*)
}

/// All arithmetic impls for given curve and corresponding scalar field types.
macro_rules! curve_arith_impl {
    ($($curve:ty, $scalar:ty)*) => ($(
        add_impl! { $curve }
        sub_impl! { $curve }
        neg_impl! { $curve }
        mul_curve_impl! { $curve, $scalar }
        div_curve_impl! { $curve, $scalar }
        add_assign_impl! { $curve }
        sub_assign_impl! { $curve }
        mul_assign_curve_impl! { $curve, $scalar }
        div_assign_curve_impl! { $curve, $scalar }
    )*)
}
