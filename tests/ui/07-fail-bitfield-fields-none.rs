#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use const_bitfield::bitfield;

bitfield! {
    struct BitNone(u8);
    bool, _, _: 0;
}

bitfield! {
    struct BitsNone(u8);
    u8, _, _: 0;
}

pub fn main() {}
