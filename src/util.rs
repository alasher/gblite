// Split a two byte dword into a two-byte pair, little endian.
// Ex: 0xFF11 -> (0x11, 0xFF)
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
