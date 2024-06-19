pub mod connection;
pub mod pixmap;
pub mod request;
pub mod response;
pub mod screen;

pub static VENDOR: &str = "Xaugh X Server";

#[derive(Debug)]
pub struct Connection {
    pub endianness: Endianness,
    pub sequence_number: u16,
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Endianness {
    Big = b'B',
    Little = b'l',
}

impl Connection {
    pub fn pad(s: usize) -> usize {
        (4 - (s % 4)) % 4
    }

    pub fn card32(&self, bytes: &[u8]) -> u32 {
        match self.endianness {
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

    pub fn card16(&self, bytes: &[u8]) -> u16 {
        match self.endianness {
            Endianness::Little => (bytes[0] as u16) | ((bytes[1] as u16) << 8),
            Endianness::Big => (bytes[1] as u16) | ((bytes[0] as u16) << 8),
        }
    }

    pub fn int16(&self, bytes: &[u8]) -> i16 {
        self.card16(bytes) as i16
    }

    pub fn copy8to32(&self, source: &[u8]) -> Vec<u32> {
        let mut target = vec![];
        for chunk in source.chunks(4) {
            target.push(self.card32(chunk));
        }
        target
    }

    pub fn to_bytes_32(&self, val: u32) -> [u8; 4] {
        match self.endianness {
            Endianness::Little => val.to_le_bytes(),
            Endianness::Big => val.to_be_bytes(),
        }
    }

    pub fn to_bytes_16(&self, val: u16) -> [u8; 2] {
        match self.endianness {
            Endianness::Little => val.to_le_bytes(),
            Endianness::Big => val.to_be_bytes(),
        }
    }
}
