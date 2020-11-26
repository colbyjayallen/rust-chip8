pub fn generate_last_byte((_a, _b, c, d): (u8, u8, u8, u8)) -> u8 {
    (c << 4) + d
}

pub fn get_tribble((_a, b, c, d): (u8, u8, u8, u8)) -> u16 {
    ((b as u16) << 8) + ((c as u16) << 4) + (d as u16)
}
