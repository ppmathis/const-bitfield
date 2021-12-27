#![no_std]
#![deny(missing_docs)]
#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

//! This crate provides macros to generate bitfield-like structs with const support.
//!
//! Due to offering const support, this library requires the usage of Rust nightly.
//! Additionally, you must add the following feature flags to your crate root:
//!
//! ```rust
//! #![feature(const_mut_refs)]
//! #![feature(const_trait_impl)]
//! ```
//!
//! This is required as some required features are currently gated behind these flags.
//! Further documentation about usage can be found in the individual macros.

/// This macro defines a new bitfield-like `struct` backed by a single uint-like type.
/// A variable amount of getters and or setters can be specified on a bitwise level.
/// Every operation automatically ensures that no bounds are being violated.
///
/// # Example
/// ```rust
/// #![feature(const_mut_refs)]
/// #![feature(const_trait_impl)]
///
/// use const_bitfield::bitfield;
///
/// bitfield! {
///     pub struct BitField(u16);
///     u8, field1, set_field1: 7, 0;   // u8 getter/setter for bits 0..=7
///     bool, field2, set_field2: 8;    // bool getter/setter for bit 8
///     bool, field3, _: 9;             // bool getter for bit 9
///     bool, _, set_field4: 10;        // bool setter for bit 10
///     u8, field5, _: 12, 11;          // u8 getter for bits 11..=12
///     u8, _, set_field6: 15, 13;      // u8 setter for bits 13..=15
/// }
/// ```
#[macro_export]
macro_rules! bitfield {
    // Generate new bitfield with getters and setters
    ($(#[$attributes:meta])* $visibility:vis struct $name:ident($type:ty); $($fields:tt)*) => {
        $(#[$attributes])*
        $visibility struct $name(pub $type);

        $crate::bitfield! {@impl_range struct $name($type)}
        impl $name {
            $crate::bitfield! {@fields @getter $($fields)*}
            $crate::bitfield! {@fields @setter $($fields)*}
        }
    };

    // Impl: Implement BitRange<T> and BitRangeMut<T> for struct(pub T)
    (@impl_range struct $name:ident($type:ty)) => {
        impl<T> const $crate::BitRange<T> for $name
        where
            $type: ~const $crate::BitRange<T>
        {
            #[inline]
            fn bits(&self, msb: usize, lsb: usize) -> T {
                self.0.bits(msb, lsb)
            }
        }

        impl<T> const $crate::BitRangeMut<T> for $name
        where
            $type: ~const $crate::BitRange<T> + ~const $crate::BitRangeMut<T>
        {
            #[inline]
            fn set_bits(&mut self, msb: usize, lsb: usize, value: T) -> &mut Self {
                self.0.set_bits(msb, lsb, value);
                self
            }
        }
    };

    // Fields: Process each field one-by-one by splitting list head off
    (@fields @$variant:tt $(#[$attributes:meta])* $visibility:vis $type:ty, $getter:tt, $setter:tt: $($exprs:expr),*; $($rest:tt)*) => {
        $crate::bitfield! {@field @$variant $(#[$attributes])* $visibility $type, $type, $type, $getter, $setter: $($exprs),*}
        $crate::bitfield! {@fields @$variant $($rest)*}
    };

    // Fields: Stop case once all fields are processed
    (@fields @$variant:tt) => {};

    // Field: Propagate field with getter and setter to individual macros
    (@field @$variant:tt $(#[$attributes:meta])* $visibility:vis $type:ty, $from:ty, $into:ty, $getter:ident, $setter:ident: $($exprs:expr),*) => {
        $crate::bitfield! {@field @$variant $(#[$attributes])* $visibility $type, $from, $into, $getter, _: $($exprs),*}
        $crate::bitfield! {@field @$variant $(#[$attributes])* $visibility $type, $from, $into, _, $setter: $($exprs),*}
    };

    // Field Getter: Bit Range
    (@field @getter $(#[$attributes:meta])* $visibility:vis $type:ty, $from:ty, $into:ty, $getter:ident, _: $msb:expr, $lsb:expr) => {
        $(#[$attributes])*
        $visibility const fn $getter(&self) -> $into {
            use $crate::BitRange;
            let raw_value: $type = self.bits($msb, $lsb);
            raw_value
        }
    };

    // Field Getter: Single Bit
    (@field @getter $(#[$attributes:meta])* $visibility:vis $type:ty, $from:ty, $into:ty, $getter:ident, _: $bit:expr) => {
        $(#[$attributes])*
        $visibility const fn $getter(&self) -> bool {
            use $crate::Bit;
            self.bit($bit)
        }
    };

    // Field Getter: Disabled
    (@field @getter $(#[$attributes:meta])* $visibility:vis $type:ty, $from:ty, $into:ty, _, $setter:ident: $($exprs:expr),*) => {};

    // Field Setter: Bit Range
    (@field @setter $(#[$attributes:meta])* $visibility:vis $type:ty, $from:ty, $into:ty, _, $setter:ident: $msb:expr, $lsb:expr) => {
        $(#[$attributes])*
        $visibility const fn $setter(&mut self, value: $from) -> &mut Self {
            use $crate::BitRangeMut;
            self.set_bits($msb, $lsb, value)
        }
    };

    // Field Setter: Single Bit
    (@field @setter $(#[$attributes:meta])* $visibility:vis $type:ty, $from:ty, $into:ty, _, $setter:ident: $bit:expr) => {
        $(#[$attributes])*
        $visibility const fn $setter(&mut self, value: bool) -> &mut Self {
            use $crate::BitMut;
            self.set_bit($bit, value)
        }
    };

    // Field Setter: Disabled
    (@field @setter $(#[$attributes:meta])* $visibility:vis $type:ty, $from:ty, $into:ty, $getter:ident, _: $($exprs:expr),*) => {};
}

/// A trait to retrieve a range of bits as type `V`.
pub trait BitRange<V> {
    /// Get a range of bits between `lsb..=msb` and return as type `V`.
    fn bits(&self, msb: usize, lsb: usize) -> V;
}

/// A trait to set a range of bits with the type `V`.
pub trait BitRangeMut<V>: BitRange<V> {
    /// Set a range of bits between `lsb..=msb` using value `V`.
    fn set_bits(&mut self, msb: usize, lsb: usize, value: V) -> &mut Self;
}

/// A trait to retrieve a single bit as a boolean.
pub trait Bit {
    /// Get a single bit and return as boolean. (`true` = set, `false` = clear)
    fn bit(&self, bit: usize) -> bool;
}

/// A trait to set a single bit as a boolean.
pub trait BitMut: Bit {
    /// Set a single bit using a boolean. (`true` = set, `false` = clear)
    fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self;
}

impl<T: ~const BitRange<u8>> const Bit for T {
    fn bit(&self, bit: usize) -> bool {
        self.bits(bit, bit) != 0
    }
}

impl<T: ~const BitRange<u8> + ~const BitRangeMut<u8>> const BitMut for T {
    fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self {
        self.set_bits(bit, bit, value as u8)
    }
}

macro_rules! impl_bitrange {
    // implement given range types for each storage type
    ($variant:tt, ($storage_type:ty, $($rest:ty),*), ($($range_type:ty),*)) => {
        impl_bitrange! {$variant, ($storage_type), ($($range_type),*)}
        impl_bitrange! {$variant, ($($rest),*), ($($range_type),*)}
    };

    // implement given range types for storage type
    ($variant:tt, ($storage_type:ty), ($($range_type:ty),*)) => {
        $(impl_bitrange! {$variant, $storage_type, $range_type})*
    };

    // implement bit range for uint-based storage type
    (uint, $storage_type:ty, $range_type:ty) => {
        impl const BitRange<$range_type> for $storage_type {
            #[inline]
            fn bits(&self, msb: usize, lsb: usize) -> $range_type {
                // treat both range bounds as inclusive
                let msb = msb + 1;

                // determine number of bits
                let storage_bits = ::core::mem::size_of::<$storage_type>() * 8;
                let range_bits = ::core::mem::size_of::<$range_type>() * 8;

                // check input range boundaries
                assert!(lsb < storage_bits, "lsb is out of bounds for bit range");
                assert!(msb <= storage_bits, "msb is out of bounds for bit range");
                assert!(lsb <= msb, "lsb must not be greater than msb for bit range");
                assert!((msb - lsb) <= range_bits, "value truncated in bit range operation");

                // shift away unnecessary high and low bits
                (*self << (storage_bits - msb) >> (storage_bits - msb) >> lsb) as $range_type
            }
        }

        impl const BitRangeMut<$range_type> for $storage_type {
            #[inline]
            fn set_bits(&mut self, msb: usize, lsb: usize, value: $range_type) -> &mut Self {
                // treat both range bounds as inclusive
                let msb = msb + 1;

                // determine number of bits
                let storage_bits = ::core::mem::size_of::<$storage_type>() * 8;

                // check range boundaries
                assert!(lsb < storage_bits, "lsb is out of bounds for bit range");
                assert!(msb <= storage_bits, "msb is out of bounds for bit range");
                assert!(lsb < msb, "lsb must not be greater than msb for bit range");

                // ensure value does not get truncated
                let new_value = value as $storage_type;
                let dropped_bits = storage_bits - (msb - lsb);
                assert!(
                    (new_value << dropped_bits >> dropped_bits) as $range_type == value,
                    "value truncated in bit range operation"
                );

                // calculate mask for clearing bits
                let mask = !((!0 as $storage_type) << (storage_bits - msb) >> (storage_bits - msb) >> lsb << lsb);

                // clear bits and OR with new value
                *self = (*self & mask) | (new_value << lsb);
                self
            }
        }
    };
}

impl_bitrange! {uint, (u8, u16, u32, u64, u128), (u8, u16, u32, u64, u128)}
impl_bitrange! {uint, (u8, u16, u32, u64, u128), (i8, i16, i32, i64, i128)}
