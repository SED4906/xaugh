use std::io::{Read, Write};

use crate::{
    pixmap::DEFAULT_PIXMAP_FORMATS,
    screen::{Depth, Screen, Visual},
    VENDOR,
};

#[derive(Debug)]
pub struct Connection<T: Read + Write> {
    pub stream: T,
    pub endianness: Endianness,
    pub sequence_number: u16,
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Endianness {
    Big = b'B',
    Little = b'l',
}

pub fn pad(s: usize) -> usize {
    (4 - (s % 4)) % 4
}

impl Endianness {
    pub fn card32(&self, bytes: &[u8]) -> u32 {
        match self {
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
        match self {
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
        match self {
            Endianness::Little => val.to_le_bytes(),
            Endianness::Big => val.to_be_bytes(),
        }
    }

    pub fn to_bytes_16(&self, val: u16) -> [u8; 2] {
        match self {
            Endianness::Little => val.to_le_bytes(),
            Endianness::Big => val.to_be_bytes(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct ConnClientPrefix {
    endian: u8,
    //pad0: u8,
    major: u16,
    minor: u16,
    n_auth_name: u16,
    d_auth_data: u16,
    //pad1: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct ConnSetupPrefix {
    pub success: u8,
    pub length_reason: u8,
    pub major: u16,
    pub minor: u16,
    pub additional_length: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct ConnSetup {
    pub release: u32,
    pub rid_base: u32,
    pub rid_mask: u32,
    pub motion_buffer_size: u32,
    pub v_bytes_vendor: u16,
    pub max_request_size: u16,
    pub num_roots: u8,
    pub num_formats: u8,
    pub image_byte_order: u8,
    pub bitmap_bit_order: u8,
    pub bitmap_scanline_unit: u8,
    pub bitmap_scanline_pad: u8,
    pub min_keycode: u8,
    pub max_keycode: u8,
    pub pad2: u32,
}

pub fn establish_connection<T: Read + Write>(mut stream: T) -> Option<Connection<T>> {
    let mut client_prefix_bytes = [0u8; 12];
    stream.read(&mut client_prefix_bytes).ok()?;
    let endianness = match client_prefix_bytes[0] {
        b'B' => Endianness::Big,
        b'l' => Endianness::Little,
        _ => panic!("invalid endianness {}", client_prefix_bytes[0]),
    };

    let client_prefix = ConnClientPrefix {
        endian: client_prefix_bytes[0],
        major: endianness.card16(&client_prefix_bytes[2..]),
        minor: endianness.card16(&client_prefix_bytes[4..]),
        n_auth_name: endianness.card16(&client_prefix_bytes[6..]),
        d_auth_data: endianness.card16(&client_prefix_bytes[8..]),
    };

    //let _auth_bytes = stream.read(&mut vec![0u8;(client_prefix.n_auth_name+pad(client_prefix.n_auth_name as usize) as u16+client_prefix.d_auth_data+pad(client_prefix.d_auth_data as usize) as u16) as usize]);

    /* Append Connection Setup */
    let conn_setup = ConnSetup {
        release: 1,
        rid_base: 0x4600000,
        rid_mask: 0x01fffff,
        motion_buffer_size: 256,
        v_bytes_vendor: VENDOR.len() as u16,
        max_request_size: 65535,
        num_roots: 1,
        num_formats: 1,
        image_byte_order: 0,
        bitmap_bit_order: 0,
        bitmap_scanline_unit: 32,
        bitmap_scanline_pad: 32,
        min_keycode: 8,
        max_keycode: 255,
        pad2: 0,
    };

    let mut additional_data: Vec<u8> = vec![];
    additional_data.append(&mut endianness.to_bytes_32(conn_setup.release).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(conn_setup.rid_base).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(conn_setup.rid_mask).to_vec());
    additional_data.append(
        &mut endianness
            .to_bytes_32(conn_setup.motion_buffer_size)
            .to_vec(),
    );
    additional_data.append(&mut endianness.to_bytes_16(conn_setup.v_bytes_vendor).to_vec());
    additional_data.append(&mut endianness.to_bytes_16(conn_setup.max_request_size).to_vec());
    additional_data.append(&mut vec![
        conn_setup.num_roots,
        conn_setup.num_formats,
        conn_setup.image_byte_order,
        conn_setup.bitmap_bit_order,
        conn_setup.bitmap_scanline_unit,
        conn_setup.bitmap_scanline_pad,
        conn_setup.min_keycode,
        conn_setup.max_keycode,
    ]);
    additional_data.append(&mut endianness.to_bytes_32(conn_setup.pad2).to_vec());
    additional_data.append(&mut VENDOR.as_bytes().to_vec());
    additional_data.append(&mut vec![0u8; pad(VENDOR.len())]);

    /* Append Pixmap Formats */
    for default_pixmap_format in DEFAULT_PIXMAP_FORMATS {
        additional_data.append(&mut vec![
            default_pixmap_format.depth,
            default_pixmap_format.bpp,
            default_pixmap_format.scanline_pad,
            default_pixmap_format.pad0,
        ]);
        additional_data.append(&mut endianness.to_bytes_32(default_pixmap_format.pad1).to_vec());
    }

    /* Define some defaults */

    let screen = Screen {
        root_window: 1,
        default_colormap: 1,
        white_pixel: 1,
        black_pixel: 0,
        current_input_masks: 0,
        width_px: 1920,
        height_px: 1080,
        width_mm: 192,
        height_mm: 108,
        min_installed_maps: 1,
        max_installed_maps: 1,
        root_visual: 1,
        backing_stores: 0,
        save_unders: 0,
        root_depth: 1,
        num_depths: 1,
    };
    let depth = Depth {
        depth: 1,
        pad0: 0,
        number_of_visuals: 1,
        pad1: 0,
    };
    let visual = Visual {
        visual_id: 1,
        class: 4,
        bits_per_rgb_val: 32,
        colormap_entries: 256,
        red_mask: 0xFF,
        green_mask: 0xFF00,
        blue_mask: 0xFF0000,
        pad0: 0,
    };

    /* Append Screens */

    additional_data.append(&mut endianness.to_bytes_32(screen.root_window).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(screen.default_colormap).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(screen.white_pixel).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(screen.black_pixel).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(screen.current_input_masks).to_vec());
    additional_data.append(&mut endianness.to_bytes_16(screen.width_px).to_vec());
    additional_data.append(&mut endianness.to_bytes_16(screen.height_px).to_vec());
    additional_data.append(&mut endianness.to_bytes_16(screen.width_mm).to_vec());
    additional_data.append(&mut endianness.to_bytes_16(screen.height_mm).to_vec());
    additional_data.append(&mut endianness.to_bytes_16(screen.min_installed_maps).to_vec());
    additional_data.append(&mut endianness.to_bytes_16(screen.max_installed_maps).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(screen.root_visual).to_vec());
    additional_data.append(&mut vec![
        screen.backing_stores,
        screen.save_unders,
        screen.root_depth,
        screen.num_depths,
    ]);

    /* Append Depths */

    additional_data.append(&mut vec![depth.depth, depth.pad0]);
    additional_data.append(&mut endianness.to_bytes_16(depth.number_of_visuals).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(depth.pad1).to_vec());

    /* Append Visuals */

    additional_data.append(&mut endianness.to_bytes_32(visual.visual_id).to_vec());
    additional_data.append(&mut vec![visual.class, visual.bits_per_rgb_val]);
    additional_data.append(&mut endianness.to_bytes_16(visual.colormap_entries).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(visual.red_mask).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(visual.green_mask).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(visual.blue_mask).to_vec());
    additional_data.append(&mut endianness.to_bytes_32(visual.pad0).to_vec());

    /* Write Connection Setup Data */
    let conn_setup_prefix = ConnSetupPrefix {
        success: 1,
        length_reason: 0,
        major: client_prefix.major,
        minor: client_prefix.minor,
        additional_length: additional_data.len() as u16 / 4,
    };

    let mut prefix_data = vec![conn_setup_prefix.success, conn_setup_prefix.length_reason];
    prefix_data.append(&mut endianness.to_bytes_16(conn_setup_prefix.major).to_vec());
    prefix_data.append(&mut endianness.to_bytes_16(conn_setup_prefix.major).to_vec());
    prefix_data.append(
        &mut endianness
            .to_bytes_16(conn_setup_prefix.additional_length)
            .to_vec(),
    );

    stream.write(&prefix_data).ok();
    stream.write(&additional_data).ok()?;

    Some(Connection {
        stream,
        endianness,
        sequence_number: 0,
    })
}

impl<T: Read + Write> Connection<T> {
    pub fn card32(&self, bytes: &[u8]) -> u32 {
        self.endianness.card32(bytes)
    }

    pub fn card16(&self, bytes: &[u8]) -> u16 {
        self.endianness.card16(bytes)
    }

    pub fn int16(&self, bytes: &[u8]) -> i16 {
        self.card16(bytes) as i16
    }

    pub fn copy8to32(&self, source: &[u8]) -> Vec<u32> {
        self.endianness.copy8to32(source)
    }

    pub fn to_bytes_32(&self, val: u32) -> [u8; 4] {
        self.endianness.to_bytes_32(val)
    }

    pub fn to_bytes_16(&self, val: u16) -> [u8; 2] {
        self.endianness.to_bytes_16(val)
    }
}
