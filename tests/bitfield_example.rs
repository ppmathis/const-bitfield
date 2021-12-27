#![feature(const_convert)] // optional, when using from/into conversion
#![feature(const_mut_refs)] // always required
#![feature(const_trait_impl)] // always required

use const_bitfield::bitfield;

bitfield! {
    #[derive(Copy, Clone)]
    pub struct MyBitField(u32);
    u8, hello, set_hello: 6, 0;         // hello is stored in bits 0..=6
    bool, world, set_world: 7;          // world is stored in bit 7
    // bits 8..=15 are unused
    u16, goodbye, set_goodbye: 31, 16;   // goodbye is stored in bits 16..=31
}

#[test]
fn test_example() {
    let mut bf = MyBitField(0);

    bf.set_hello(0b0110110);
    bf.set_world(true);
    bf.set_goodbye(0xF00F);

    println!("{}", bf.hello());
    println!("{}", bf.world());
    println!("{}", bf.goodbye());
}
