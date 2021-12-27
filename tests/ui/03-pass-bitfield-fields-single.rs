#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use const_bitfield::bitfield;

bitfield! {
    struct SingleOne(u8);
    bool, bit_0, set_bit_0: 0;
}

bitfield! {
    struct SingleMany(u8);
    bool, bit_0, _: 0;
    bool, _, set_bit_1: 1;
    bool, bit_2, set_bit_2: 2;
    bool, bit_6, set_bit_6: 6;
}

bitfield! {
    struct RangeOne(u8);
    u8, bit_3_0, set_bit_3_0: 3, 0;
}

bitfield! {
    struct RangeMany(u8);
    u8, bit_2_0, _: 2, 0;
    u8, _, set_bit_5_3: 5, 3;
    u8, bit_7_6, set_bit_7_6: 7, 6;
}

bitfield! {
    struct Mixed(u16);
    bool, bit_0, set_bit_0: 0;
    u8, bit_4_1, set_bit_4_1: 4, 1;
    bool, bit_5, _: 5;
    u8, _, set_bit_7_6: 7, 6;
    bool, _, set_bit_8: 8;
    u8, bit_11_9, _: 11, 9;
    bool, bit_12, set_bit_12: 12;
    u8, bit_15_13, set_bit_15_13: 15, 13;
}

fn main() {}
