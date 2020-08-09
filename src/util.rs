#![allow(dead_code)]

use chrono::{Utc, Datelike, Timelike};

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

pub fn create_file_name(suffix: &str) -> String {
    let dt = Utc::now();
    format!("gblite_{}_{:02}_{:02}_{}{}.log", dt.year(), dt.month(),dt.day(),
            dt.num_seconds_from_midnight(), suffix)
}
