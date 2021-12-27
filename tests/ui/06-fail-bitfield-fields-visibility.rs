#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

mod inner {
    use const_bitfield::bitfield;

    bitfield! {
        pub struct Test(u8);
        u8, test, set_test: 3, 0;
    }
}

use inner::Test;

pub fn main() {
    let mut test = Test(0);
    test.set_test(0xF);
}
