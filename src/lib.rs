//! Utilities for swapping the byte-order of in-memory types.

#![no_std]
#![warn(missing_docs)]

mod core_impls;

/// Enumeration providing byte-order variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    /// Little-endian.
    Le,
    /// Big-endian.
    Be,
}

impl ByteOrder {
    /// Implementation for computing the value for [`NATIVE`](ByteOrder::NATIVE).
    ///
    /// Providing the value through the invocation of a function like this supresses Cargo's
    /// documentation generation from providing a documented value for the constant. This is
    /// desirable because it avoids suggesting to the user that the constant's value is always equal
    /// to whatever the native byte-order was for the machine that generated the documentation.
    const fn compute_native() -> Self {
        #[cfg(target_endian = "little")]
        {
            Self::Le
        }
        #[cfg(target_endian = "big")]
        {
            Self::Be
        }
    }

    /// The compilation target's native byte-order.
    pub const NATIVE: Self = Self::compute_native();

    /// The opposite of the compilation target's native byte-order.
    pub const NATIVE_OPPOSITE: Self = Self::NATIVE.opposite();

    /// Checks if `self` is the compilation target's native byte-order.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use lilbig::ByteOrder;
    /// assert_eq!(ByteOrder::NATIVE.is_native(), true);
    /// assert_eq!(ByteOrder::NATIVE_OPPOSITE.is_native(), false);
    /// assert_eq!(ByteOrder::Le.is_native(), cfg!(target_endian = "little"));
    /// assert_eq!(ByteOrder::Be.is_native(), cfg!(target_endian = "big"));
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_native(self) -> bool {
        matches!(self, Self::NATIVE)
    }

    /// Retrieves the opposite byte-order of `self`.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// use lilbig::ByteOrder;
    /// assert_eq!(ByteOrder::NATIVE.opposite(), ByteOrder::NATIVE_OPPOSITE);
    /// assert_eq!(ByteOrder::NATIVE_OPPOSITE.opposite(), ByteOrder::NATIVE);
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

    /// Invokes [`opposite()`](Self::opposite).
    #[inline(always)]
    #[must_use]
    fn not(self) -> Self::Output {
        self.opposite()
    }
}

/// Trait for converting the byte-order of primitive-esque types.
///
/// # Examples
/// Implementing:
/// ```
/// /// 256-bit unsigned integer.
/// #[derive(Clone, Copy)]
/// #[repr(align(32))]
/// struct U256([u8; 32]);
///
/// impl lilbig::ByteOrdered for U256 {
///     fn swapped_order(mut self) -> Self {
///         self.0.reverse();
///         self
///     }
/// }
/// ```
pub trait ByteOrdered: Sized {
    /// Unconditionally swap the byte-order of `self`.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // Implementing a helper function that can generically provide the value of `1` encoded in a
    /// // byte-order provided at runtime.
    /// use lilbig::{ByteOrder, ByteOrdered};
    ///
    /// /// Trait generically providing the value of `1` for some type.
    /// trait One: Sized { const ONE: Self; }
    /// impl One for u32 { const ONE: Self = 1; }
    ///
    /// /// Helper function to construct a value of `1` encoded in an argument provided byte-order.
    /// fn one_ordered<T: One + ByteOrdered>(byte_order: ByteOrder) -> T {
    ///     let mut one = T::ONE;
    ///     if !byte_order.is_native() {
    ///         one = one.swapped_order();
    ///     }
    ///     one
    /// }
    ///
    /// const NE_ONE: u32 = 1u32;
    /// const LE_ONE: u32 = 1u32.to_le();
    /// const BE_ONE: u32 = 1u32.to_be();
    ///
    /// assert_eq!(NE_ONE, one_ordered::<u32>(ByteOrder::NATIVE));
    /// assert_eq!(LE_ONE, one_ordered::<u32>(ByteOrder::Le));
    /// assert_eq!(BE_ONE, one_ordered::<u32>(ByteOrder::Be));
    /// ```
    #[must_use]
    fn swapped_order(self) -> Self;

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s bytes so that it is encoded in the machine's native byte-order.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // Converting the byte-order of a value to the machine's native byte-order where the value's
    /// // current byte-order is provided as an argument.
    /// use lilbig::{ByteOrder, ByteOrdered};
    ///
    /// const NE_N: u32 = 0x7cf3a4b1;
    /// const LE_N: u32 = NE_N.to_le();
    /// const BE_N: u32 = NE_N.to_be();
    ///
    /// assert_eq!(NE_N, NE_N.ordered_ne(ByteOrder::NATIVE));
    /// assert_eq!(NE_N, LE_N.ordered_ne(ByteOrder::Le));
    /// assert_eq!(NE_N, BE_N.ordered_ne(ByteOrder::Be));
    /// ```
    #[inline]
    #[must_use]
    fn ordered_ne(self, current_order: ByteOrder) -> Self {
        match current_order {
            ByteOrder::NATIVE => self,
            ByteOrder::NATIVE_OPPOSITE => self.swapped_order(),
        }
    }

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s bytes so that it is encoded in little-endian byte-order.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // Converting the byte-order of a value to little-endian where the value's current
    /// // byte-order is provided as an argument.
    /// use lilbig::{ByteOrder, ByteOrdered};
    ///
    /// const NE_N: u32 = 0x7cf3a4b1;
    /// const LE_N: u32 = NE_N.to_le();
    /// const BE_N: u32 = NE_N.to_be();
    ///
    /// assert_eq!(LE_N, NE_N.ordered_le(ByteOrder::NATIVE));
    /// assert_eq!(LE_N, LE_N.ordered_le(ByteOrder::Le));
    /// assert_eq!(LE_N, BE_N.ordered_le(ByteOrder::Be));
    /// ```
    #[inline]
    #[must_use]
    fn ordered_le(self, current_order: ByteOrder) -> Self {
        match current_order {
            ByteOrder::Le => self,
            ByteOrder::Be => self.swapped_order(),
        }
    }

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s bytes so that it is encoded in big-endian byte-order.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // Converting the byte-order of a value to big-endian where the value's current byte-order
    /// // is provided as an argument.
    /// use lilbig::{ByteOrder, ByteOrdered};
    ///
    /// const NE_N: u32 = 0x7cf3a4b1;
    /// const LE_N: u32 = NE_N.to_le();
    /// const BE_N: u32 = NE_N.to_be();
    ///
    /// assert_eq!(BE_N, NE_N.ordered_be(ByteOrder::NATIVE));
    /// assert_eq!(BE_N, LE_N.ordered_be(ByteOrder::Le));
    /// assert_eq!(BE_N, BE_N.ordered_be(ByteOrder::Be));
    #[inline]
    #[must_use]
    fn ordered_be(self, current_order: ByteOrder) -> Self {
        match current_order {
            ByteOrder::Be => self,
            ByteOrder::Le => self.swapped_order(),
        }
    }

    /// Provided `self`'s current byte-order and a new byte-order for `self`, conditionally swap
    /// `self`'s bytes so that it is encoded in that new byte-order.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // Converting the byte-order of a value to various different byte-orders where the value's
    /// // current byte-order and new byte-order are both provided as arguments.
    /// use lilbig::{ByteOrder, ByteOrdered};
    ///
    /// const NE_N: u32 = 0x7cf3a4b1;
    /// const LE_N: u32 = NE_N.to_le();
    /// const BE_N: u32 = NE_N.to_be();
    ///
    /// assert_eq!(NE_N, NE_N.ordered_as(ByteOrder::NATIVE, ByteOrder::NATIVE));
    /// assert_eq!(LE_N, NE_N.ordered_as(ByteOrder::NATIVE, ByteOrder::Le));
    /// assert_eq!(BE_N, NE_N.ordered_as(ByteOrder::NATIVE, ByteOrder::Be));
    ///
    /// assert_eq!(NE_N, LE_N.ordered_as(ByteOrder::Le, ByteOrder::NATIVE));
    /// assert_eq!(LE_N, LE_N.ordered_as(ByteOrder::Le, ByteOrder::Le));
    /// assert_eq!(BE_N, LE_N.ordered_as(ByteOrder::Le, ByteOrder::Be));
    ///
    /// assert_eq!(NE_N, BE_N.ordered_as(ByteOrder::Be, ByteOrder::NATIVE));
    /// assert_eq!(LE_N, BE_N.ordered_as(ByteOrder::Be, ByteOrder::Le));
    /// assert_eq!(BE_N, BE_N.ordered_as(ByteOrder::Be, ByteOrder::Be));
    /// ```
    #[inline]
    #[must_use]
    fn ordered_as(self, current_order: ByteOrder, new_order: ByteOrder) -> Self {
        if current_order == new_order {
            self
        } else {
            self.swapped_order()
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
/// Implementing:
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
/// impl FieldsByteOrdered for GroupId {
///     fn swap_field_orders(&mut self) {
///         self.0.swap_field_orders();
///     }
/// }
///
/// impl FieldsByteOrdered for AccessPrivilegeFlags {
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
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // Defining a record describing a person and swapping the byte-order of an instances's
    /// // fields.
    /// use lilbig::FieldsByteOrdered;
    ///
    /// /// Record describing a person.
    /// struct PersonRecord {
    ///     /// The person's name.
    ///     pub name: String,
    ///     /// The person's age.
    ///     pub age: u16,
    /// }
    ///
    /// impl FieldsByteOrdered for PersonRecord {
    ///     fn swap_field_orders(&mut self) {
    ///         // `age` is the only field to which byte-order is relevant.
    ///         self.age.swap_field_orders();
    ///     }
    /// }
    ///
    /// const NAME: &'static str = "Wolfgang";
    /// const AGE: u16 = 23u16;
    /// let mut record = PersonRecord {
    ///     name: NAME.into(),
    ///     age: AGE,
    /// };
    ///
    /// record.swap_field_orders();
    /// assert_eq!(NAME, record.name);
    /// assert_eq!(AGE.swap_bytes(), record.age);
    /// ```
    fn swap_field_orders(&mut self);

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s fields so that they are in the machine's native byte-order.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // Converting the byte-order of all the elements within an array to the machine's native
    /// // byte-order where the current byte-order of the elements is provided as an argument.
    /// use lilbig::{ByteOrder, FieldsByteOrdered};
    ///
    /// const NE_NUMBERS: [u32; 4] = [0x7cf3a4b1, 0x3dd4f42, 0xff317cde, 0x87fce321];
    /// let le_numbers = NE_NUMBERS.map(u32::to_le);
    /// let be_numbers = NE_NUMBERS.map(u32::to_be);
    ///
    /// /// Helper function to clone a value and then encode the fields of that clone in the
    /// /// machine's native byte-order.
    /// fn clone_ordered_ne<T: Clone + FieldsByteOrdered>(
    ///    value: &T,
    ///    current_order: ByteOrder,
    /// ) -> T {
    ///     let mut new_value = value.clone();
    ///     new_value.order_fields_ne(current_order);
    ///     new_value
    /// }
    ///
    /// assert_eq!(NE_NUMBERS, clone_ordered_ne(&NE_NUMBERS, ByteOrder::NATIVE));
    /// assert_eq!(NE_NUMBERS, clone_ordered_ne(&le_numbers, ByteOrder::Le));
    /// assert_eq!(NE_NUMBERS, clone_ordered_ne(&be_numbers, ByteOrder::Be));
    /// ```
    #[inline]
    fn order_fields_ne(&mut self, current_order: ByteOrder) {
        if current_order != ByteOrder::NATIVE {
            self.swap_field_orders();
        }
    }

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s fields so that they are in little-endian byte-order.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // Converting the byte-order of all the elements within an array to little-endian where the
    /// // current byte-order of the elements is provided as an argument.
    /// use lilbig::{ByteOrder, FieldsByteOrdered};
    ///
    /// const NE_NUMBERS: [u32; 4] = [0x7cf3a4b1, 0x3dd4f42, 0xff317cde, 0x87fce321];
    /// let le_numbers = NE_NUMBERS.map(u32::to_le);
    /// let be_numbers = NE_NUMBERS.map(u32::to_be);
    ///
    /// /// Helper function to clone a value and then encode the fields of that clone in
    /// /// little-endian.
    /// fn clone_ordered_le<T: Clone + FieldsByteOrdered>(
    ///    value: &T,
    ///    current_order: ByteOrder,
    /// ) -> T {
    ///     let mut new_value = value.clone();
    ///     new_value.order_fields_le(current_order);
    ///     new_value
    /// }
    ///
    /// assert_eq!(le_numbers, clone_ordered_le(&NE_NUMBERS, ByteOrder::NATIVE));
    /// assert_eq!(le_numbers, clone_ordered_le(&le_numbers, ByteOrder::Le));
    /// assert_eq!(le_numbers, clone_ordered_le(&be_numbers, ByteOrder::Be));
    /// ```
    #[inline]
    fn order_fields_le(&mut self, current_order: ByteOrder) {
        if current_order != ByteOrder::Le {
            self.swap_field_orders();
        }
    }

    /// Provided `self`'s current byte-order as an input argument, conditionally swap the byte-order
    /// of `self`'s fields so that they are in big-endian byte-order.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // Converting the byte-order of all the elements within an array to big-endian where the
    /// // current byte-order of the elements is provided as an argument.
    /// use lilbig::{ByteOrder, FieldsByteOrdered};
    ///
    /// const NE_NUMBERS: [u32; 4] = [0x7cf3a4b1, 0x3dd4f42, 0xff317cde, 0x87fce321];
    /// let le_numbers = NE_NUMBERS.map(u32::to_le);
    /// let be_numbers = NE_NUMBERS.map(u32::to_be);
    ///
    /// /// Helper function to clone a value and then encode the fields of that clone in big-endian.
    /// fn clone_ordered_be<T: Clone + FieldsByteOrdered>(
    ///    value: &T,
    ///    current_order: ByteOrder,
    /// ) -> T {
    ///     let mut new_value = value.clone();
    ///     new_value.order_fields_be(current_order);
    ///     new_value
    /// }
    ///
    /// assert_eq!(be_numbers, clone_ordered_be(&NE_NUMBERS, ByteOrder::NATIVE));
    /// assert_eq!(be_numbers, clone_ordered_be(&le_numbers, ByteOrder::Le));
    /// assert_eq!(be_numbers, clone_ordered_be(&be_numbers, ByteOrder::Be));
    /// ```
    #[inline]
    fn order_fields_be(&mut self, current_order: ByteOrder) {
        if current_order != ByteOrder::Be {
            self.swap_field_orders();
        }
    }

    /// Provided `self`'s current byte-order and a new byte-order for `self`, conditionally swap
    /// the byte-order of `self`'s fields so that they are encoded in that new byte-order.
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // Converting the byte-order of all the elements within an array to various byte-orders
    /// // where the current byte-order of the elements and the new byte-order for the elements are
    /// // both provided as arguments.
    /// use lilbig::{ByteOrder, FieldsByteOrdered};
    ///
    /// const NE_NUMBERS: [u32; 4] = [0x7cf3a4b1, 0x3dd4f42, 0xff317cde, 0x87fce321];
    /// let le_numbers = NE_NUMBERS.map(u32::to_le);
    /// let be_numbers = NE_NUMBERS.map(u32::to_be);
    ///
    /// /// Helper function to clone a value and then encode the fields of that clone in a new
    /// /// byte-order.
    /// fn clone_ordered_as<T: Clone + FieldsByteOrdered>(
    ///    value: &T,
    ///    current_order: ByteOrder,
    ///    new_order: ByteOrder,
    /// ) -> T {
    ///     let mut new_value = value.clone();
    ///     new_value.order_fields_as(current_order, new_order);
    ///     new_value
    /// }
    ///
    /// assert_eq!(NE_NUMBERS, clone_ordered_as(&NE_NUMBERS, ByteOrder::NATIVE, ByteOrder::NATIVE));
    /// assert_eq!(le_numbers, clone_ordered_as(&NE_NUMBERS, ByteOrder::NATIVE, ByteOrder::Le));
    /// assert_eq!(be_numbers, clone_ordered_as(&NE_NUMBERS, ByteOrder::NATIVE, ByteOrder::Be));
    ///
    /// assert_eq!(NE_NUMBERS, clone_ordered_as(&le_numbers, ByteOrder::Le, ByteOrder::NATIVE));
    /// assert_eq!(le_numbers, clone_ordered_as(&le_numbers, ByteOrder::Le, ByteOrder::Le));
    /// assert_eq!(be_numbers, clone_ordered_as(&le_numbers, ByteOrder::Le, ByteOrder::Be));
    ///
    /// assert_eq!(NE_NUMBERS, clone_ordered_as(&be_numbers, ByteOrder::Be, ByteOrder::NATIVE));
    /// assert_eq!(le_numbers, clone_ordered_as(&be_numbers, ByteOrder::Be, ByteOrder::Le));
    /// assert_eq!(be_numbers, clone_ordered_as(&be_numbers, ByteOrder::Be, ByteOrder::Be));
    /// ```
    fn order_fields_as(&mut self, current_order: ByteOrder, new_order: ByteOrder) {
        if current_order != new_order {
            self.swap_field_orders();
        }
    }
}
