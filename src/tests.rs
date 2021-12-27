extern crate std;

use crate::{BitRange, BitRangeMut};
use std::panic;

macro_rules! impl_test_range_uint {
    ($name:ident, $type:ty, $signed_type:ty, [$($smaller_types:ty),*], [$($larger_types:ty),*]) => {
        mod $name {
            use super::*;

            const BIT_LENGTH: usize = ::core::mem::size_of::<$type>() * 8;
            const MAX_VALUE: $type = <$type>::MAX;
            const MAX_MSB: usize = BIT_LENGTH - 1;

            #[test]
            pub fn test_bits_all() {
                let result: $type = MAX_VALUE.bits(MAX_MSB, 0);
                assert_eq!(result, MAX_VALUE);
            }

            #[test]
            pub fn test_bits_single() {
                for bit in 0..BIT_LENGTH {
                    let value: $type = 0b1 << bit;
                    expect_group_of_bits(value, bit, bit);
                }
            }

            #[test]
            pub fn test_bits_multiple() {
                for lsb in 0..(BIT_LENGTH - 3) {
                    let value: $type = 0b111 << lsb;
                    expect_group_of_bits(value, lsb + 2, lsb);
                }
            }

            #[test]
            #[should_panic]
            pub fn test_bits_invalid_lsb() {
                BitRange::<$type>::bits(&MAX_VALUE, BIT_LENGTH, BIT_LENGTH);
            }

            #[test]
            #[should_panic]
            pub fn test_bits_invalid_msb() {
                BitRange::<$type>::bits(&MAX_VALUE, BIT_LENGTH, 0);
            }

            #[test]
            #[should_panic]
            pub fn test_bits_invalid_msb_lsb() {
                BitRange::<$type>::bits(&MAX_VALUE, 0, MAX_MSB);
            }

            #[test]
            pub fn test_bits_into_signed() {
                BitRange::<$signed_type>::bits(&MAX_VALUE, MAX_MSB, 0);
            }

            #[test]
            pub fn test_bits_into_larger() {
                $(
                    BitRange::<$larger_types>::bits(&MAX_VALUE, MAX_MSB, 0);
                )*
            }

            #[test]
            pub fn test_bits_into_smaller() {
                $(
                    let small_bit_length = ::core::mem::size_of::<$smaller_types>() * 8;
                    let small_max_msb = small_bit_length - 1;
                    BitRange::<$smaller_types>::bits(&MAX_VALUE, MAX_MSB, MAX_MSB - small_max_msb);
                )*
            }

            #[test]
            pub fn test_bits_invalid_into_smaller() {
                $(
                    let result = panic::catch_unwind(|| {
                        BitRange::<$smaller_types>::bits(&MAX_VALUE, MAX_MSB, 0);
                    });
                    assert!(result.is_err());
                )*
            }

            #[test]
            pub fn test_set_bits_all() {
                let mut value: $type = 0;
                value.set_bits(MAX_MSB, 0, MAX_VALUE);
                assert_eq!(value, MAX_VALUE);
            }

            #[test]
            pub fn test_set_bits_single() {
                for bit in 0..BIT_LENGTH {
                    let mut value: $type = 0;
                    value.set_bits(bit, bit, 0b1);
                    expect_group_of_bits(value, bit, bit);
                }
            }

            #[test]
            pub fn test_set_bits_multiple() {
                for lsb in 0..(BIT_LENGTH - 3) {
                    let mut value: $type = 0;
                    value.set_bits(lsb + 2, lsb, 0b111);
                    expect_group_of_bits(value, lsb + 2, lsb);
                }
            }

            #[test]
            #[should_panic]
            pub fn test_set_bits_invalid_lsb() {
                let mut value: $type = 0;
                value.set_bits(BIT_LENGTH, BIT_LENGTH, 0);
            }

            #[test]
            #[should_panic]
            pub fn test_set_bits_invalid_msb() {
                let mut value: $type = 0;
                value.set_bits(BIT_LENGTH, 0, 0);
            }

            #[test]
            #[should_panic]
            pub fn test_set_bits_invalid_msb_lsb() {
                let mut value: $type = 0;
                value.set_bits(0, MAX_MSB, 0);
            }

            #[test]
            pub fn test_set_bits_truncated_value() {
                $(
                    let result = panic::catch_unwind(|| {
                        let mut value: $type = 0;
                        value.set_bits(MAX_MSB, 0, <$larger_types>::MAX);
                    });
                    assert!(result.is_err());
                )*
            }

            fn expect_group_of_bits(value: $type, msb: usize, lsb: usize) {
                // ensure leading bits are clear
                if lsb > 0 {
                    let leading_bits: $type = value.bits(lsb - 1, 0);
                    assert_eq!(leading_bits, 0);
                }

                // ensure targeted bits are set
                let shifted_bits: usize = BIT_LENGTH - (msb + 1 - lsb);
                let expected_bits: $type = (!0 as $type) << shifted_bits >> shifted_bits;
                let actual_bits: $type = value.bits(msb, lsb);
                assert_eq!(actual_bits, expected_bits);

                // ensure trailing bits are clear
                if msb < MAX_MSB {
                    let trailing_bits: $type = value.bits(MAX_MSB, msb + 1);
                    assert_eq!(trailing_bits, 0);
                }
            }
        }
    };
}

impl_test_range_uint!(type_u8, u8, i8, [], [u16, u32, u64, u128]);
impl_test_range_uint!(type_u16, u16, i16, [u8], [u32, u64, u128]);
impl_test_range_uint!(type_u32, u32, i32, [u8, u16], [u64, u128]);
impl_test_range_uint!(type_u64, u64, i64, [u8, u16, u32], [u128]);
impl_test_range_uint!(type_u128, u128, i128, [u8, u16, u32, u64], []);
