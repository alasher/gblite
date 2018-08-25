#![allow(dead_code)]

// Split a two byte dword into a two-byte pair, little endian.
// Ex: 0xFF11 -> (0x11, 0xFF)
pub fn join_u8(pair: (u8, u8)) -> u16 {
    pair.0 as u16 | ((pair.1 as u16) << 8)
}

pub fn split_u16(dword: u16) -> (u8, u8) {
    ((dword & 0xFF) as u8, (dword >> 8) as u8)
}

pub fn set_bit(word: u8, bit: u8, set: bool) -> u8 {
    let mask = (1 as u8) << bit;
    if set { (word & !mask) | mask } else { word & !mask }
}

pub fn is_bit_set(word: u8, bit: u8) -> bool {
    (word & (1 << bit)) != 0
}

// Trait to convert from i8 into another generic primitive, see CPU::alu.
// After scouring Stack Overflow, I couldn't find a better way to solve this problem.
pub trait FromI8 {
    fn from_i8(val: i8) -> Self;
}

impl FromI8 for u8 {
    fn from_i8(val: i8) -> u8 {
        val as u8
    }
}

impl FromI8 for u16 {
    fn from_i8(val: i8) -> u16 {
        val as u16
    }
}
