#![feature(const_convert)]
#![feature(const_option)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use const_bitfield::bitfield;

const KERNEL_CODE64: u64 = 0x00AF9B000000FFFF;
const KERNEL_CODE32: u64 = 0x00CF9B000000FFFF;
const KERNEL_DATA: u64 = 0x00CF93000000FFFF;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DescriptorType {
    SystemSegment = 0,
    UserSegment = 1,
}

impl const From<bool> for DescriptorType {
    fn from(value: bool) -> Self {
        match value {
            false => DescriptorType::SystemSegment,
            true => DescriptorType::UserSegment,
        }
    }
}

impl const From<DescriptorType> for bool {
    fn from(value: DescriptorType) -> Self {
        match value {
            DescriptorType::SystemSegment => false,
            DescriptorType::UserSegment => true,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SegmentType {
    DataReadOnly = 0b000,
    DataReadWrite = 0b001,
    DataReadOnlyDown = 0b010,
    DataReadWriteDown = 0b011,
    CodeExecOnly = 0b100,
    CodeExecRead = 0b101,
    CodeExecOnlyConforming = 0b110,
    CodeExecReadConforming = 0b111,
}

impl const From<u8> for SegmentType {
    fn from(value: u8) -> Self {
        match value {
            x if x == SegmentType::DataReadOnly as u8 => SegmentType::DataReadOnly,
            x if x == SegmentType::DataReadWrite as u8 => SegmentType::DataReadWrite,
            x if x == SegmentType::DataReadOnlyDown as u8 => SegmentType::DataReadOnlyDown,
            x if x == SegmentType::DataReadWriteDown as u8 => SegmentType::DataReadWriteDown,
            x if x == SegmentType::CodeExecOnly as u8 => SegmentType::CodeExecOnly,
            x if x == SegmentType::CodeExecRead as u8 => SegmentType::CodeExecRead,
            x if x == SegmentType::CodeExecOnlyConforming as u8 => {
                SegmentType::CodeExecOnlyConforming
            }
            x if x == SegmentType::CodeExecReadConforming as u8 => {
                SegmentType::CodeExecReadConforming
            }
            _ => panic!("invalid value for segment type"),
        }
    }
}

impl const From<SegmentType> for u8 {
    fn from(value: SegmentType) -> Self {
        value as u8
    }
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct Descriptor(u64);
    u32, limit_0_15, set_limit_0_15: 15, 0;
    u32, base_0_15, set_base_0_15: 31, 16;
    u8, base_16_23, set_base_16_23: 39, 32;
    bool, accessed, set_accessed: 40;
    u8, from into SegmentType, segment_type, set_segment_type: 43, 41;
    bool, from into DescriptorType, descriptor_type, set_descriptor_type: 44;
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
    assert_eq!(descriptor.segment_type(), SegmentType::CodeExecRead);
    assert_eq!(descriptor.descriptor_type(), DescriptorType::UserSegment);
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
        .set_segment_type(SegmentType::CodeExecRead)
        .set_descriptor_type(DescriptorType::UserSegment)
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
    assert_eq!(descriptor.segment_type(), SegmentType::CodeExecRead);
    assert_eq!(descriptor.descriptor_type(), DescriptorType::UserSegment);
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
        .set_segment_type(SegmentType::CodeExecRead)
        .set_descriptor_type(DescriptorType::UserSegment)
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
    assert_eq!(descriptor.segment_type(), SegmentType::DataReadWrite);
    assert_eq!(descriptor.descriptor_type(), DescriptorType::UserSegment);
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
        .set_segment_type(SegmentType::DataReadWrite)
        .set_descriptor_type(DescriptorType::UserSegment)
        .set_privilege_level(0)
        .set_present(true)
        .set_long_mode(false)
        .set_size_flag(true)
        .set_granularity(true);

    assert_eq!(descriptor.0, KERNEL_DATA);
}
