#![no_std]

mod core_impls;

/// Byte-order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    /// Little endian.
    Le,
    /// Big endian.
    Be,
}

impl ByteOrder {
    /// The compilation target's native byte-order.
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

    /// Returns `true` if `self` is the compilation target's native byte-order.
    ///
    /// # Examples
    /// ```
    /// use lilbig::ByteOrder;
    /// assert_eq!(ByteOrder::NATIVE.is_native(), true);
    /// assert_eq!(ByteOrder::Le.is_native(), cfg!(target_endian = "little"));
    /// assert_eq!(ByteOrder::Be.is_native(), cfg!(target_endian = "big"));
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_native(self) -> bool {
        matches!(self, Self::NATIVE)
    }

    /// Retrieve the opposite byte-order of `self`.
    ///
    /// # Examples
    /// ```
    /// use lilbig::ByteOrder;
    /// assert_eq!(ByteOrder::Le.opposite(), ByteOrder::Be);
    /// assert_eq!(ByteOrder::Be.opposite(), ByteOrder::Le);
    /// ```
    #[inline]
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Le => Self::Be,
            Self::Be => Self::Le,
        }
    }
}

impl core::ops::Not for ByteOrder {
    type Output = Self;

    #[inline(always)]
    #[must_use]
    fn not(self) -> Self::Output {
        self.opposite()
    }
}

/// Trait for converting the byte-order of primitive-esque types.
///
/// # Examples
/// ```
/// /// 256-bit unsigned integer.
/// #[derive(Clone, Copy)]
/// #[repr(align(32))]
/// struct U256([u8; 32]);
///
/// impl lilbig::ByteOrdered for U256 {
///     fn swapped_order(self) -> Self {
///         let mut bytes = self.0;
///         bytes.reverse();
///         Self(bytes)
///     }
/// }
/// ```
pub trait ByteOrdered: Sized {
    /// Unconditionally swap the byte-order of `self`.
    ///
    /// # Examples
    /// ```
    /// use lilbig::{ByteOrder, ByteOrdered};
    ///
    /// /// Trait generically providing the value of `1` for some type.
    /// trait One: Sized { const ONE: Self; }
    /// impl One for u32 { const ONE: Self = 1; }
    ///
    /// /// Construct a value of `1` encoded with the given byte-order.
    /// fn ordered_one<T: One + ByteOrdered>(byte_order: ByteOrder) -> T {
    ///     let mut one = T::ONE;
    ///     if !byte_order.is_native() {
    ///         one = one.swapped_order();
    ///     }
    ///     one
    /// }
    ///
    /// assert_eq!(ordered_one::<u32>(ByteOrder::NATIVE), 1u32);
    /// assert_eq!(ordered_one::<u32>(ByteOrder::Le), u32::from_ne_bytes([1, 0, 0, 0]));
    /// assert_eq!(ordered_one::<u32>(ByteOrder::Be), u32::from_ne_bytes([0, 0, 0, 1]));
    /// ```
    #[must_use]
    fn swapped_order(self) -> Self;

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s bytes so that it is encoded in the machine's native byte-order.
    ///
    /// # Examples
    /// ```
    /// use lilbig::{ByteOrder, ByteOrdered};
    ///
    /// const LE_ONE: u32 = u32::from_ne_bytes([1, 0, 0, 0]); // `1u32` encoded in little endian.
    /// const BE_ONE: u32 = u32::from_ne_bytes([0, 0, 0, 1]); // `1u32` encoded in big endian.
    ///
    /// assert_eq!(1u32.ordered_ne(ByteOrder::NATIVE), 1u32);
    /// assert_eq!(LE_ONE.ordered_ne(ByteOrder::Le), 1u32);
    /// assert_eq!(BE_ONE.ordered_ne(ByteOrder::Be), 1u32);
    /// ```
    #[inline]
    #[must_use]
    fn ordered_ne(self, current_order: ByteOrder) -> Self {
        match current_order {
            ByteOrder::NATIVE => self,
            _ => self.swapped_order(),
        }
    }

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s bytes so that it is encoded in little-endian byte-order.
    ///
    /// # Examples
    /// ```
    /// use lilbig::{ByteOrder, ByteOrdered};
    ///
    /// const LE_ONE: u32 = u32::from_ne_bytes([1, 0, 0, 0]); // `1u32` encoded in little endian.
    /// const BE_ONE: u32 = u32::from_ne_bytes([0, 0, 0, 1]); // `1u32` encoded in big endian.
    ///
    /// assert_eq!(1u32.ordered_le(ByteOrder::NATIVE), LE_ONE);
    /// assert_eq!(LE_ONE.ordered_le(ByteOrder::Le), LE_ONE);
    /// assert_eq!(BE_ONE.ordered_le(ByteOrder::Be), LE_ONE);
    /// ```
    #[inline]
    #[must_use]
    fn ordered_le(self, current_order: ByteOrder) -> Self {
        match current_order {
            ByteOrder::Le => self,
            _ => self.swapped_order(),
        }
    }

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s bytes so that it is encoded in big-endian byte-order.
    ///
    /// # Examples
    /// ```
    /// use lilbig::{ByteOrder, ByteOrdered};
    ///
    /// const LE_ONE: u32 = u32::from_ne_bytes([1, 0, 0, 0]); // `1u32` encoded in little endian.
    /// const BE_ONE: u32 = u32::from_ne_bytes([0, 0, 0, 1]); // `1u32` encoded in big endian.
    ///
    /// assert_eq!(1u32.ordered_be(ByteOrder::NATIVE), BE_ONE);
    /// assert_eq!(LE_ONE.ordered_be(ByteOrder::Le), BE_ONE);
    /// assert_eq!(BE_ONE.ordered_be(ByteOrder::Be), BE_ONE);
    #[inline]
    #[must_use]
    fn ordered_be(self, current_order: ByteOrder) -> Self {
        match current_order {
            ByteOrder::Be => self,
            _ => self.swapped_order(),
        }
    }
}

/// Trait for converting the byte-order of a type whose fields are all encoded in one byte-order.
///
/// This is implemented for primitives, arrays, and slices by default to facilitate easy nesting.
///
/// Calling [`swap_field_orders()`](FieldsByteOrdered::swap_field_orders) on a primitive type will
/// merely swap that primitive's byte-order.
///
/// Calling [`swap_field_orders()`](FieldsByteOrdered::swap_field_orders) on an array or slice will
/// swap the byte-order of all that array/slice's elements.
///
/// # Examples
/// ```
/// // Defining the low level types for a basic user implemented file-system where the byte-order of
/// // the file-system can vary.
/// use lilbig::FieldsByteOrdered;
///
/// /// The maximum groups supported by the file-system.
/// const MAX_GROUPS: usize = 16;
///
/// /// Index identifier for a group within the file-system.
/// #[repr(transparent)]
/// struct GroupId(pub u32);
///
/// /// Bit-flags specifying access privileges in the file-system.
/// #[repr(transparent)]
/// struct AccessPrivilegeFlags(pub u32);
///
/// /// Information describing a file in the file-system.
/// #[repr(C)]
/// struct FileInfo {
///     /// Unix epoch time stamp for when the file was last accessed.
///     pub accessed_time_stamp: u64,
///     /// Unix epoch time stamp for when the file was last modified.
///     pub modified_time_stamp: u64,
///     /// Byte offset to the bytes of the file on the underlying disk.
///     pub offset: u64,
///     /// Byte length of the file.
///     pub length: u64,
///     /// Byte capacity reserved for the file.
///     pub capacity: u64,
///     /// Per-group access privileges for the file.
///     pub group_access_privileges: [AccessPrivilegeFlags; MAX_GROUPS],
/// }
///
/// impl FieldsByteOrdered for AccessPrivilegeFlags {
///     fn swap_field_orders(&mut self) {
///         self.0.swap_field_orders();
///     }
/// }
///
/// impl FieldsByteOrdered for GroupId {
///     fn swap_field_orders(&mut self) {
///         self.0.swap_field_orders();
///     }
/// }
///
/// impl FieldsByteOrdered for FileInfo {
///     fn swap_field_orders(&mut self) {
///         self.accessed_time_stamp.swap_field_orders();
///         self.modified_time_stamp.swap_field_orders();
///         self.offset.swap_field_orders();
///         self.length.swap_field_orders();
///         self.capacity.swap_field_orders();
///         self.group_access_privileges.swap_field_orders();
///     }
/// }
/// ```
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
        if current_order != ByteOrder::Be {
            self.swap_field_orders();
        }
    }
}
