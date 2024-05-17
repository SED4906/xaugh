use connection::{Connection, Endianness};

pub mod connection;
pub mod pixmap;
pub mod request;
pub mod screen;
pub mod response;

pub static VENDOR: &str = "Xaugh X Server";

pub fn pad(s: usize) -> usize {
    (4 - (s % 4)) % 4
}

pub fn card32(connection: &Connection, bytes: &[u8]) -> u32 {
    match connection.endianness {
        Endianness::Little => {
            (bytes[0] as u32)
                | ((bytes[1] as u32) << 8)
                | ((bytes[2] as u32) << 16)
                | ((bytes[3] as u32) << 24)
        }
        Endianness::Big => {
            (bytes[3] as u32)
                | ((bytes[2] as u32) << 8)
                | ((bytes[1] as u32) << 16)
                | ((bytes[0] as u32) << 24)
        }
    }
}

pub fn card16(connection: &Connection, bytes: &[u8]) -> u16 {
    match connection.endianness {
        Endianness::Little => (bytes[0] as u16) | ((bytes[1] as u16) << 8),
        Endianness::Big => (bytes[1] as u16) | ((bytes[0] as u16) << 8),
    }
}

pub fn int16(connection: &Connection, bytes: &[u8]) -> i16 {
    card16(connection, bytes) as i16
}

pub fn copy8to32(connection: &Connection, source: &[u8]) -> Vec<u32> {
    let mut target = vec![];
    for chunk in source.chunks(4) {
        target.push(card32(connection, chunk));
    }
    target
}
