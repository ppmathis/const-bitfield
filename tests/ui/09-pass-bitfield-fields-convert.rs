#![feature(const_convert)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use const_bitfield::bitfield;

bitfield! {
    struct ConvertBit(u8);
    bool, bit_raw, set_bit_raw: 0;
    bool, from YesNoField, bit_from, set_bit_from: 1;
    bool, into YesNoField, bit_into, set_bit_into: 2;
    bool, from into YesNoField, bit_both, set_bit_both: 3;
    bool, from TrueFalseField, into YesNoField, bit_mixed_1, set_bit_mixed_1: 4;
    bool, from YesNoField, into TrueFalseField, bit_mixed_2, set_bit_mixed_2: 5;
}

bitfield! {
    struct ConvertBits(u16);
    u8, bits_raw, set_bits_raw: 1, 0;
    u8, from NumField1, bits_from, set_bits_from: 3, 2;
    u8, into NumField1, bits_into, set_bits_into: 5, 4;
    u8, from into NumField1, bits_both, set_bits_both: 7, 6;
    u8, from NumField2, into NumField1, bits_mixed_1, set_bits_mixed_1: 9, 8;
    u8, from NumField1, into NumField2, bits_mixed_2, set_bits_mixed_2: 11, 10;
}

pub fn main() {
    test_convert_bit();
    test_convert_bits();
}

fn test_convert_bit() {
    // test from conversions of single bit
    let bf = ConvertBit(0);
    let _result: bool = bf.bit_raw();
    let _result: bool = bf.bit_from();
    let _result: YesNoField = bf.bit_into();
    let _result: YesNoField = bf.bit_both();
    let _result: YesNoField = bf.bit_mixed_1();
    let _result: TrueFalseField = bf.bit_mixed_2();

    // test into conversions of single bit
    let mut bf = ConvertBit(0);
    bf.set_bit_raw(true);
    bf.set_bit_from(YesNoField::Yes);
    bf.set_bit_into(true);
    bf.set_bit_both(YesNoField::Yes);
    bf.set_bit_mixed_1(TrueFalseField::True);
    bf.set_bit_mixed_2(YesNoField::Yes);
}

fn test_convert_bits() {
    // test from conversions of single bit
    let bf = ConvertBits(0);
    let _result: u8 = bf.bits_raw();
    let _result: u8 = bf.bits_from();
    let _result: NumField1 = bf.bits_into();
    let _result: NumField1 = bf.bits_both();
    let _result: NumField1 = bf.bits_mixed_1();
    let _result: NumField2 = bf.bits_mixed_2();

    // test into conversions of single bit
    let mut bf = ConvertBits(0);
    bf.set_bits_raw(1);
    bf.set_bits_from(NumField1::A);
    bf.set_bits_into(1);
    bf.set_bits_both(NumField1::A);
    bf.set_bits_mixed_1(NumField2::Z);
    bf.set_bits_mixed_2(NumField1::A);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum NumField1 {
    A = 0b00,
    B = 0b01,
    C = 0b10,
    D = 0b11,
}

impl const From<u8> for NumField1 {
    fn from(value: u8) -> Self {
        match value {
            0b00 => NumField1::A,
            0b01 => NumField1::B,
            0b10 => NumField1::C,
            0b11 => NumField1::D,
            _ => panic!("invalid value"),
        }
    }
}

impl const From<NumField1> for u8 {
    fn from(value: NumField1) -> Self {
        match value {
            NumField1::A => 0b00,
            NumField1::B => 0b01,
            NumField1::C => 0b10,
            NumField1::D => 0b11,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum NumField2 {
    W = 0b00,
    X = 0b01,
    Y = 0b10,
    Z = 0b11,
}

impl const From<u8> for NumField2 {
    fn from(value: u8) -> Self {
        match value {
            0b00 => NumField2::W,
            0b01 => NumField2::X,
            0b10 => NumField2::Y,
            0b11 => NumField2::Z,
            _ => panic!("invalid value"),
        }
    }
}

impl const From<NumField2> for u8 {
    fn from(value: NumField2) -> Self {
        match value {
            NumField2::W => 0b00,
            NumField2::X => 0b01,
            NumField2::Y => 0b10,
            NumField2::Z => 0b11,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum YesNoField {
    Yes,
    No,
}

impl const From<bool> for YesNoField {
    fn from(value: bool) -> Self {
        match value {
            false => YesNoField::No,
            true => YesNoField::Yes,
        }
    }
}

impl const From<YesNoField> for bool {
    fn from(value: YesNoField) -> Self {
        match value {
            YesNoField::No => false,
            YesNoField::Yes => true,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TrueFalseField {
    True,
    False,
}

impl const From<bool> for TrueFalseField {
    fn from(value: bool) -> Self {
        match value {
            false => TrueFalseField::False,
            true => TrueFalseField::True,
        }
    }
}

impl const From<TrueFalseField> for bool {
    fn from(value: TrueFalseField) -> Self {
        match value {
            TrueFalseField::False => false,
            TrueFalseField::True => true,
        }
    }
}
