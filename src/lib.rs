pub fn pad(s: usize) -> usize {
    (4 - (s % 4)) % 4
}

pub fn card32(bytes: &[u8]) -> u32 {
    (bytes[0] as u32)
    | ((bytes[1] as u32) << 8)
    | ((bytes[2] as u32) << 16)
    | ((bytes[3] as u32) << 24)
}

pub fn card16(bytes: &[u8]) -> u16 {
    (bytes[0] as u16)
    | ((bytes[1] as u16) << 8)
}

pub fn int16(bytes: &[u8]) -> i16 {
    card16(bytes) as i16
}