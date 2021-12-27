#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use const_bitfield::bitfield;
use std::panic;

bitfield! {
    struct Test(u128);
    u8, f1, set_f1: 7, 0;
    u16, f2, set_f2: 23, 8;
    u32, f3, set_f3: 55, 24;
    u64, f4, set_f4: 119, 56;
    u8, f5, set_f5: 127, 120;
}

#[test]
pub fn test_get_multiple() {
    let test = Test(0x55_4444444444444444_33333333_2222_11);

    assert_eq!(test.f1(), 0x11, "field 1 mismatch");
    assert_eq!(test.f2(), 0x2222, "field 2 mismatch");
    assert_eq!(test.f3(), 0x3333_3333, "field 3 mismatch");
    assert_eq!(test.f4(), 0x4444_4444_4444_4444, "field 4 mismatch");
    assert_eq!(test.f5(), 0x55, "field 5 mismatch");
}

#[test]
pub fn test_set_single() {
    for idx in 1..=5 {
        let mut test = Test(0);

        // set single field to max value and check it
        set_field(&mut test, idx, u128::MAX);
        assert_eq!(get_field(&test, idx), get_field_max(idx));

        // ensure all other fields are zero
        for sub_idx in 1..=5 {
            if sub_idx != idx {
                assert_eq!(get_field(&test, sub_idx), 0);
            }
        }
    }
}

#[test]
pub fn test_set_multiple() {
    let mut test = Test(0);
    test.set_f1(0x11)
        .set_f2(0x2222)
        .set_f3(0x3333_3333)
        .set_f4(0x4444_4444_4444_4444)
        .set_f5(0x55);

    assert_eq!((test.0 >> 0) & u8::MAX as u128, 0x11, "field 1 mismatch");
    assert_eq!((test.0 >> 8) & u16::MAX as u128, 0x2222, "field 2 mismatch");
    assert_eq!(
        (test.0 >> 24) & u32::MAX as u128,
        0x3333_3333,
        "field 3 mismatch"
    );
    assert_eq!(
        (test.0 >> 56) & u64::MAX as u128,
        0x4444_4444_4444_4444,
        "field 4 mismatch"
    );
    assert_eq!((test.0 >> 120) & u8::MAX as u128, 0x55, "field 5 mismatch");
}

fn get_field_max(index: usize) -> u128 {
    match index {
        1 | 5 => u8::MAX as u128,
        2 => u16::MAX as u128,
        3 => u32::MAX as u128,
        4 => u64::MAX as u128,
        _ => panic!("invalid field index {}", index),
    }
}

fn get_field(test: &Test, index: usize) -> u128 {
    match index {
        1 => test.f1() as u128,
        2 => test.f2() as u128,
        3 => test.f3() as u128,
        4 => test.f4() as u128,
        5 => test.f5() as u128,
        _ => panic!("invalid field index {}", index),
    }
}

fn set_field(test: &mut Test, index: usize, value: u128) -> &mut Test {
    match index {
        1 => test.set_f1(value as u8),
        2 => test.set_f2(value as u16),
        3 => test.set_f3(value as u32),
        4 => test.set_f4(value as u64),
        5 => test.set_f5(value as u8),
        _ => panic!("invalid field index {}", index),
    }
}
