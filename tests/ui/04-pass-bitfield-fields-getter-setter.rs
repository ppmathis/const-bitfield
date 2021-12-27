#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use const_bitfield::bitfield;

bitfield! {
    struct BitGetterSetter(u8);
    bool, test, set_test: 0;
}

bitfield! {
    struct BitGetterOnly(u8);
    bool, test, _: 0;
}

bitfield! {
    struct BitSetterOnly(u8);
    bool, _, set_test: 0;
}

bitfield! {
    struct BitsGetterSetter(u8);
    u8, test, set_test: 3, 0;
}

bitfield! {
    struct BitsGetterOnly(u8);
    u8, test, _: 3, 0;
}

bitfield! {
    struct BitsSetterOnly(u8);
    u8, _, set_test: 3, 0;
}

fn main() {}
