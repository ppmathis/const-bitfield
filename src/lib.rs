#![no_std]
#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

#[cfg(test)]
mod tests;

pub trait BitRange<V> {
    fn bits(&self, msb: usize, lsb: usize) -> V;
}

pub trait BitRangeMut<V>: BitRange<V> {
    fn set_bits(&mut self, msb: usize, lsb: usize, value: V) -> &mut Self;
}

pub trait Bit {
    fn bit(&self, bit: usize) -> bool;
}

pub trait BitMut: Bit {
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

pub struct TestIntegral(pub u8);

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
                let mask = !((!0 as $storage_type) << (storage_bits - msb) >> (storage_bits - msb) >> lsb);

                // clear bits and OR with new value
                *self = (*self & mask) | (new_value << lsb);
                self
            }
        }
    };
}

impl_bitrange! {uint, (u8, u16, u32, u64, u128), (u8, u16, u32, u64, u128)}
impl_bitrange! {uint, (u8, u16, u32, u64, u128), (i8, i16, i32, i64, i128)}
