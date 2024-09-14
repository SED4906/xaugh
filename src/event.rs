use std::io::{Read, Write};

use crate::connection::Connection;

#[repr(C)]
#[derive(Clone, Debug)]
pub enum Event {
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelease,
    MotionNotify,
    EnterNotify,
    LeaveNotify,
    FocusIn,
    FocusOut,
    KeymapNotify,
    Expose,
    GraphicsExposure,
    NoExposure,
    VisibilityNotify,
    CreateNotify,
    DestroyNotify,
    UnmapNotify,
    MapNotify,
    MapRequest,
    ReparentNotify,
    ConfigureNotify,
    ConfigureRequest,
    GravityNotify,
    ResizeRequest,
    CirculateNotify,
    CirculateRequest,
    PropertyNotify,
    SelectionClear,
    SelectionRequest,
    SelectionNotify,
    ColormapNotify,
    ClientMessage {
        sequence_number: u16,
        window: u32,
        atom_type: u32,
        data: ClientMessageData,
    },
    MappingNotify,
}

impl<T: Read + Write> Connection<T> {
    pub fn event(&self, slice: &[u8]) -> Event {
        match slice[0] {
            33 => Event::ClientMessage {
                sequence_number: self.card16(&slice[2..]),
                window: self.card32(&slice[4..]),
                atom_type: self.card32(&slice[8..]),
                data: match slice[1] {
                    8 => ClientMessageData::Bytes(*slice[12..].first_chunk::<20>().unwrap()),
                    16 => ClientMessageData::Shorts(*slice[12..].chunks(2).map(|b| self.card16(b)).collect::<Vec<u16>>().first_chunk::<10>().unwrap()),
                    32 => ClientMessageData::Longs(*slice[12..].chunks(4).map(|b| self.card32(b)).collect::<Vec<u32>>().first_chunk::<5>().unwrap()),
                    _ => panic!("invalid ClientMessage format")
                }
            },
            _ => panic!("unknown event type")
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub enum ClientMessageData {
    Bytes([u8; 20]),
    Shorts([u16; 10]),
    Longs([u32; 5]),
}
