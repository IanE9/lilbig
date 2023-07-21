//! Implementations of [`ByteOrdered`] and [`FieldsByteOrdered`] on [`core`] types.

use crate::{ByteOrdered, FieldsByteOrdered};

/// Implement both [`ByteOrdered`] and [`FieldsByteOrdered`] as NOPs for a set of types.
macro_rules! impl_ordered_nop {
    ($($ty: ty),+) => {
        $(impl ByteOrdered for $ty {
            #[inline(always)]
            fn swapped_order(self) -> Self {
                self
            }
        }
        impl FieldsByteOrdered for $ty {
            #[inline(always)]
            fn swap_field_orders(&mut self) {}
        })+
    };
}

/// Implement both [`ByteOrdered`] and [`FieldsByteOrdered`] for a set of core integer types.
macro_rules! impl_ordered_int {
    ($($ty: ty),+) => {
        $(impl ByteOrdered for $ty {
            #[inline]
            fn swapped_order(self) -> Self {
                self.swap_bytes()
            }
        }
        impl FieldsByteOrdered for $ty {
            #[inline]
            fn swap_field_orders(&mut self) {
                *self = self.swap_bytes();
            }
        })+
    };
}

/// Implement both [`ByteOrdered`] and [`FieldsByteOrdered`] for a set of core floating point types.
macro_rules! impl_ordered_float {
    ($($ty: ty),+) => {
        $(impl ByteOrdered for $ty {
            #[inline]
            fn swapped_order(self) -> Self {
                Self::from_bits(self.to_bits().swap_bytes())
            }
        }
        impl FieldsByteOrdered for $ty {
            #[inline]
            fn swap_field_orders(&mut self) {
                *self = Self::from_bits(self.to_bits().swap_bytes());
            }
        })+
    };
}

impl_ordered_nop!(i8, u8);
impl_ordered_int!(i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
impl_ordered_float!(f32, f64);

impl<T: FieldsByteOrdered> FieldsByteOrdered for [T] {
    #[inline]
    fn swap_field_orders(&mut self) {
        self.iter_mut().for_each(T::swap_field_orders);
    }
}

impl<T: FieldsByteOrdered, const N: usize> FieldsByteOrdered for [T; N] {
    #[inline]
    fn swap_field_orders(&mut self) {
        self.iter_mut().for_each(T::swap_field_orders);
    }
}
