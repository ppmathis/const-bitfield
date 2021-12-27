#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

mod inner {
    use const_bitfield::bitfield;

    bitfield! {
        pub struct Test(u8);
        u8, test_1, set_test_1: 3, 0;
        pub u8, test_2, set_test_2: 7, 4;
    }

    pub fn test_inner() {
        let mut test = Test(0);
        test.set_test_1(0xF);
        test.set_test_2(0xF);
    }
}

use inner::{test_inner, Test};

pub fn main() {
    test_inner();

    let mut test = Test(0);
    test.set_test_2(0xF);
}
