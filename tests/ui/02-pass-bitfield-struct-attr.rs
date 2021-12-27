#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use const_bitfield::bitfield;

bitfield! {
    #[derive(Copy, Clone, Debug)]
    pub struct Test(u8);
}

fn main() {
    let value = Test(0);

    // Copy
    let copy = value;
    println!("value={} copy={}", value.0, copy.0);

    // Clone
    let clone = value.clone();
    println!("value={} clone={}", value.0, clone.0);

    // Debug
    println!("{:?}", value);
}
