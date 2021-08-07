#![allow(dead_code)]

use chrono::{Utc, Datelike, Timelike};

/// Join two u8 bytes into a single u16, little endian.
/// 
/// ```
/// let combined = join_u8((0xFF, 0x11));
/// assert_eq!(combined, 0x11FF);
/// ```
pub fn join_u8(pair: (u8, u8)) -> u16 {
    pair.0 as u16 | ((pair.1 as u16) << 8)
}

/// Split a u16 into two u8 bytes, little endian.
/// 
/// ```
/// let split = split_u16(0x32DD);
/// assert_eq!(split, (0xDD, 0x32));
/// ```
pub fn split_u16(dword: u16) -> (u8, u8) {
    ((dword & 0xFF) as u8, (dword >> 8) as u8)
}

/// Set or unset the specified bit within a byte.
/// 
/// ```
/// assert_eq!(set_bit(0xFF, 7, false), 0x7F)
/// assert_eq!(set_bit(0xF0, 4, true), 0xF0)
/// ```
pub fn set_bit(word: u8, bit: u8, set: bool) -> u8 {
    let mask = (1 as u8) << bit;
    if set { (word & !mask) | mask } else { word & !mask }
}


/// Returns true iff the specified bit in a byte is set.
/// 
/// ```
/// assert_eq!(is_bit_set(0xFF, 7), true)
/// assert_eq!(is_bit_set(0x0F, 4), false)
/// ```
pub fn is_bit_set(word: u8, bit: u8) -> bool {
    (word & (1 << bit)) != 0
}

pub fn create_file_name(suffix: &str) -> String {
    let dt = Utc::now();
    format!("gblite_{}_{:02}_{:02}_{}{}.log", dt.year(), dt.month(),dt.day(),
            dt.num_seconds_from_midnight(), suffix)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_join_u8() {
        for hib in 0..=255 {
            for lob in 0..=255 {
                println!("Hi: {}, lo: {}", hib, lob);
                assert_eq!(join_u8((lob, hib)), (hib as u16)*0x100 + lob as u16)
            }
        }
    }

    #[test]
    fn test_split_u16() {
        for hib in 0..=255 {
            for lob in 0..=255 {
                let joined = join_u8((lob, hib));
                assert_eq!(split_u16(joined), (lob, hib))
            }
        }
    }

    #[test]
    fn test_bit_set() {
        let mut word: u8 = 0;

        // Set every bit to 1, and verify it is set
        for bit in 0..=7 {
            word = set_bit(word, bit, true);
            assert_eq!(is_bit_set(word, bit), true);
        }

        // Now set every bit to 0, and verify it's unset
        for bit in 0..=7 {
            word = set_bit(word, bit, false);
            assert_eq!(is_bit_set(word, bit), false);
        }
    }
}