/// Byte-order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    /// Little endian.
    Le,
    /// Big endian.
    Be,
}

impl ByteOrder {
    /// The target's native byte-order.
    pub const NATIVE: Self = {
        #[cfg(target_endian = "little")]
        {
            ByteOrder::Le
        }
        #[cfg(target_endian = "big")]
        {
            ByteOrder::Be
        }
    };
}

/// Trait for converting the byte-order of primitives.
pub trait ByteOrdered: Sized {
    /// Unconditionally swap the byte-order of `self`.
    #[must_use]
    fn swapped_order(self) -> Self;

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s bytes so that it is in the machine's native byte-order.
    #[inline]
    #[must_use]
    fn ordered_ne(self, current_order: ByteOrder) -> Self {
        match current_order {
            ByteOrder::NATIVE => self,
            _ => self.swapped_order(),
        }
    }

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s bytes so that it is in little-endian byte-order.
    #[inline]
    #[must_use]
    fn ordered_le(self, current_order: ByteOrder) -> Self {
        match current_order {
            ByteOrder::Le => self,
            _ => self.swapped_order(),
        }
    }

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s bytes so that it is in big-endian byte-order.
    #[inline]
    #[must_use]
    fn ordered_be(self, current_order: ByteOrder) -> Self {
        match current_order {
            ByteOrder::Be => self,
            _ => self.swapped_order(),
        }
    }
}

/// Trait for converting the byte-order of a type's fields.
///
/// This is implemented for primitives and slice by default like types to facilitate easy nesting.
/// Calling [`swap_field_orders()`](FieldsByteOrdered::swap_field_orders) on a primitive type will
/// swap merely swap that primitive's byte-order.
pub trait FieldsByteOrdered {
    /// Unconditionally swap the byte-order of `self`'s fields.
    fn swap_field_orders(&mut self);

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s fields so that they are in the machine's native byte-order.
    #[inline]
    fn order_fields_ne(&mut self, current_order: ByteOrder) {
        if current_order != ByteOrder::NATIVE {
            self.swap_field_orders();
        }
    }

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s fields so that they are in little-endian byte-order.
    #[inline]
    fn order_fields_le(&mut self, current_order: ByteOrder) {
        if current_order != ByteOrder::Le {
            self.swap_field_orders();
        }
    }

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s fields so that they are in big-endian byte-order.
    #[inline]
    fn order_fields_be(&mut self, current_order: ByteOrder) {
        if current_order != ByteOrder::Le {
            self.swap_field_orders();
        }
    }
}

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

impl_ordered_int!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
impl_ordered_float!(f32, f64);

impl<T: FieldsByteOrdered> FieldsByteOrdered for [T] {
    fn swap_field_orders(&mut self) {
        self.iter_mut().for_each(T::swap_field_orders);
    }
}

impl<T: FieldsByteOrdered, const N: usize> FieldsByteOrdered for [T; N] {
    fn swap_field_orders(&mut self) {
        self.iter_mut().for_each(T::swap_field_orders);
    }
}
