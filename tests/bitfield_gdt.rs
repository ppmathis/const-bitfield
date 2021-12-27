#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use const_bitfield::bitfield;

const KERNEL_CODE64: u64 = 0x00AF9B000000FFFF;
const KERNEL_CODE32: u64 = 0x00CF9B000000FFFF;
const KERNEL_DATA: u64 = 0x00CF93000000FFFF;

bitfield! {
    #[derive(Copy, Clone)]
    pub struct Descriptor(u64);
    u32, limit_0_15, set_limit_0_15: 15, 0;
    u32, base_0_15, set_base_0_15: 31, 16;
    u8, base_16_23, set_base_16_23: 39, 32;
    bool, accessed, set_accessed: 40;
    bool, read_write, set_read_write: 41;
    bool, direction_conforming, set_direction_conforming: 42;
    bool, executable, set_executable: 43;
    bool, descriptor_type, set_descriptor_type: 44;
    u8, privilege_level, set_privilege_level: 46, 45;
    bool, present, set_present: 47;
    u8, limit_16_19, set_limit_16_19: 51, 48;
    bool, long_mode, set_long_mode: 53;
    bool, size_flag, set_size_flag: 54;
    bool, granularity, set_granularity: 55;
    u8, base_24_31, set_base_24_31: 63, 56;
}

#[test]
pub fn test_parse_kernel_code64() {
    let descriptor = Descriptor(KERNEL_CODE64);

    // base
    assert_eq!(descriptor.base_0_15(), 0);
    assert_eq!(descriptor.base_16_23(), 0);
    assert_eq!(descriptor.base_24_31(), 0);

    // limit
    assert_eq!(descriptor.limit_0_15(), 0xFFFF);
    assert_eq!(descriptor.limit_16_19(), 0xF);

    // flags
    assert_eq!(descriptor.accessed(), true);
    assert_eq!(descriptor.read_write(), true);
    assert_eq!(descriptor.direction_conforming(), false);
    assert_eq!(descriptor.executable(), true);
    assert_eq!(descriptor.descriptor_type(), true);
    assert_eq!(descriptor.privilege_level(), 0);
    assert_eq!(descriptor.present(), true);
    assert_eq!(descriptor.long_mode(), true);
    assert_eq!(descriptor.size_flag(), false);
    assert_eq!(descriptor.granularity(), true);
}

#[test]
pub fn test_build_kernel_code64() {
    let mut descriptor = Descriptor(0);
    descriptor
        // base
        .set_base_0_15(0)
        .set_base_16_23(0)
        .set_base_24_31(0)
        // limit
        .set_limit_0_15(0xFFFF)
        .set_limit_16_19(0xF)
        // flags
        .set_accessed(true)
        .set_read_write(true)
        .set_direction_conforming(false)
        .set_executable(true)
        .set_descriptor_type(true)
        .set_privilege_level(0)
        .set_present(true)
        .set_long_mode(true)
        .set_size_flag(false)
        .set_granularity(true);

    assert_eq!(descriptor.0, KERNEL_CODE64);
}

#[test]
pub fn test_parse_kernel_code32() {
    let descriptor = Descriptor(KERNEL_CODE32);

    // base
    assert_eq!(descriptor.base_0_15(), 0);
    assert_eq!(descriptor.base_16_23(), 0);
    assert_eq!(descriptor.base_24_31(), 0);

    // limit
    assert_eq!(descriptor.limit_0_15(), 0xFFFF);
    assert_eq!(descriptor.limit_16_19(), 0xF);

    // flags
    assert_eq!(descriptor.accessed(), true);
    assert_eq!(descriptor.read_write(), true);
    assert_eq!(descriptor.direction_conforming(), false);
    assert_eq!(descriptor.executable(), true);
    assert_eq!(descriptor.descriptor_type(), true);
    assert_eq!(descriptor.privilege_level(), 0);
    assert_eq!(descriptor.present(), true);
    assert_eq!(descriptor.long_mode(), false);
    assert_eq!(descriptor.size_flag(), true);
    assert_eq!(descriptor.granularity(), true);
}

#[test]
pub fn test_build_kernel_code32() {
    let mut descriptor = Descriptor(0);
    descriptor
        // base
        .set_base_0_15(0)
        .set_base_16_23(0)
        .set_base_24_31(0)
        // limit
        .set_limit_0_15(0xFFFF)
        .set_limit_16_19(0xF)
        // flags
        .set_accessed(true)
        .set_read_write(true)
        .set_direction_conforming(false)
        .set_executable(true)
        .set_descriptor_type(true)
        .set_privilege_level(0)
        .set_present(true)
        .set_long_mode(false)
        .set_size_flag(true)
        .set_granularity(true);

    assert_eq!(descriptor.0, KERNEL_CODE32);
}

#[test]
pub fn test_parse_kernel_data() {
    let descriptor = Descriptor(KERNEL_DATA);

    // base
    assert_eq!(descriptor.base_0_15(), 0);
    assert_eq!(descriptor.base_16_23(), 0);
    assert_eq!(descriptor.base_24_31(), 0);

    // limit
    assert_eq!(descriptor.limit_0_15(), 0xFFFF);
    assert_eq!(descriptor.limit_16_19(), 0xF);

    // flags
    assert_eq!(descriptor.accessed(), true);
    assert_eq!(descriptor.read_write(), true);
    assert_eq!(descriptor.direction_conforming(), false);
    assert_eq!(descriptor.executable(), false);
    assert_eq!(descriptor.descriptor_type(), true);
    assert_eq!(descriptor.privilege_level(), 0);
    assert_eq!(descriptor.present(), true);
    assert_eq!(descriptor.long_mode(), false);
    assert_eq!(descriptor.size_flag(), true);
    assert_eq!(descriptor.granularity(), true);
}

#[test]
pub fn test_build_kernel_data() {
    let mut descriptor = Descriptor(0);
    descriptor
        // base
        .set_base_0_15(0)
        .set_base_16_23(0)
        .set_base_24_31(0)
        // limit
        .set_limit_0_15(0xFFFF)
        .set_limit_16_19(0xF)
        // flags
        .set_accessed(true)
        .set_read_write(true)
        .set_direction_conforming(false)
        .set_executable(false)
        .set_descriptor_type(true)
        .set_privilege_level(0)
        .set_present(true)
        .set_long_mode(false)
        .set_size_flag(true)
        .set_granularity(true);

    assert_eq!(descriptor.0, KERNEL_DATA);
}
