# const-bitfield

[![GitHub](https://img.shields.io/static/v1?label=GitHub&message=const-bitfield&style=for-the-badge&logo=github&logoColor=white&color=informational)](https://github.com/ppmathis/const-bitfield)
[![Crates.io](https://img.shields.io/crates/v/const-bitfield?style=for-the-badge&logo=rust&logoColor=white&color=informational)](https://crates.io/crates/const-bitfield)
[![docs.rs](https://img.shields.io/docsrs/const-bitfield?style=for-the-badge&label=docs.rs&logo=rust&logoColor=white&color=informational)](https://docs.rs/const-bitfield)
[![GitHub Actions Status](https://img.shields.io/github/workflow/status/ppmathis/const-bitfield/CI/main?style=for-the-badge&logo=githubactions&logoColor=white)](https://github.com/ppmathis/const-bitfield/actions)

This crate provides a `bitfield!` macro for generating bitfield-like structures in Rust with support for compile-time
evaluation using `const`. The following features are currently supported:

- Support of `u8`, `u16`, `u32`, `u64`, `u128` as backing storage types
- Get and set single-bit values as `bool`
- Get and set values as unsigned / signed integer types
- Optional mapping of individual getter to any custom type using `From` trait
- Optional mapping of individual setter from any custom type using `From` trait
- Optional support for overlapping fields for union-like behavior
- Overlapping of fields for union-like implementations
- Compatibility with `no_std`
- Usage of arbitrary attributes on struct and fields
- Usage of arbitrary visibility modifiers on struct and fields

Unfortunately Rust Stable does not currently contain all required features for implementing this crate.
To use of this library, you must use a recent Rust Nightly release and add the following feature flags to your crate root:

```rust
#![feature(const_convert)]      // optional, when using from/into conversion
#![feature(const_mut_refs)]     // always required
#![feature(const_trait_impl)]   // always required
```

Here is a simple example of how this library can be used:

```rust
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

fn example() {
    let mut bf = MyBitField(0);

    bf.set_hello(0b0110110);
    bf.set_world(true);
    bf.set_goodbye(0xF00F);

    println!("{}", bf.hello());
    println!("{}", bf.world());
    println!("{}", bf.goodbye());
}
```

A more detailed example can be found within [tests/bitfield_gdt.rs](tests/bitfield_gdt.rs) which uses the `bitfield!` macro
to implement parsing and building entries of the [x86 Global Descriptor Table](https://en.wikipedia.org/wiki/Global_Descriptor_Table).

You may wish to combine this crate with [const-enum](https://crates.io/crates/const-enum) to directly map fields of your bitfield
from and into enums with a `repr` type. To do so, simply use `#[derive(ConstEnum)]` along with e.g. `repr(u8)`. This specific use case
is also shown as part of the GDT example linked above.

## Additional Credits
This crate is heavily inspired by [dzamlo/rust-bitfield](https://github.com/dzamlo/rust-bitfield).

The API between these two crates is similar, but no compatibility is guaranteed.
Unlike the other library, this one focuses on `const`-support to allow using it as a helper for complex data structures
at compile-time without having an impact on runtime performance.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
