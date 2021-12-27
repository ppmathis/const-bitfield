#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use const_bitfield::bitfield;

bitfield! {
    struct Vis(u8);
}

bitfield! {
    pub struct VisPub(u8);
}

bitfield! {
    pub(crate) struct VisPubCrate(u8);
}

bitfield! {
    pub(self) struct VisPubSelf(u8);
}

mod inner {
    use super::*;

    bitfield! {
        pub(super) struct VisPubSuper(u8);
    }
}

fn main() {}
