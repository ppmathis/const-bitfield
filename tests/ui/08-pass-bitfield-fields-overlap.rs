#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use const_bitfield::bitfield;

bitfield! {
    struct Test(u8);
    bool, b0, set_b0: 0;
    bool, b6, set_b6: 6;

    u8, b0_3, set_b0_3: 3, 0;
    u8, b4_7, set_b4_7: 7, 4;
}

pub fn main() {}
