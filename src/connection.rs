use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{card16, pad};

use crate::{
    pixmap::{PixmapFormat, DEFAULT_PIXMAP_FORMATS},
    screen::{Depth, Screen, Visual},
    VENDOR,
};

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

pub fn establish_connection(mut stream: &TcpStream) -> Option<Connection> {
    let mut client_prefix_bytes = [0u8; 12];
    stream.read(&mut client_prefix_bytes).ok()?;
    let endianness = match client_prefix_bytes[0] {
        b'B' => Endianness::Big,
        b'l' => Endianness::Little,
        _ => panic!("invalid endianness {}", client_prefix_bytes[0]),
    };
    let client_prefix = ConnClientPrefix {
        endian: client_prefix_bytes[0],
        major: card16(
            &Connection {
                endianness: endianness.clone(),
                sequence_number: 0,
            },
            &client_prefix_bytes[2..],
        ),
        minor: card16(
            &Connection {
                endianness: endianness.clone(),
                sequence_number: 0,
            },
            &client_prefix_bytes[4..],
        ),
        n_auth_name: card16(
            &Connection {
                endianness: endianness.clone(),
                sequence_number: 0,
            },
            &client_prefix_bytes[6..],
        ),
        d_auth_data: card16(
            &Connection {
                endianness: endianness.clone(),
                sequence_number: 0,
            },
            &client_prefix_bytes[8..],
        ),
    };

    //let _auth_bytes = stream.read(&mut vec![0u8;(client_prefix.n_auth_name+pad(client_prefix.n_auth_name as usize) as u16+client_prefix.d_auth_data+pad(client_prefix.d_auth_data as usize) as u16) as usize]);
    let mut additional_data: Vec<u8> = vec![];
    additional_data.append(
        &mut unsafe {
            ::core::slice::from_raw_parts(
                (&ConnSetup {
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
                } as *const ConnSetup) as *const u8,
                ::core::mem::size_of::<ConnSetup>(),
            )
        }
        .to_vec(),
    );
    additional_data.append(&mut VENDOR.as_bytes().to_vec());
    additional_data.append(&mut vec![0u8; pad(VENDOR.len())]);
    additional_data.append(
        &mut unsafe {
            ::core::slice::from_raw_parts(
                (&DEFAULT_PIXMAP_FORMATS[0] as *const PixmapFormat) as *const u8,
                ::core::mem::size_of::<PixmapFormat>(),
            )
        }
        .to_vec(),
    );
    additional_data.append(
        &mut unsafe {
            ::core::slice::from_raw_parts(
                (&Screen {
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
                } as *const Screen) as *const u8,
                ::core::mem::size_of::<Screen>(),
            )
        }
        .to_vec(),
    );
    additional_data.append(
        &mut unsafe {
            ::core::slice::from_raw_parts(
                (&Depth {
                    depth: 1,
                    pad0: 0,
                    number_of_visuals: 1,
                    pad1: 0,
                } as *const Depth) as *const u8,
                ::core::mem::size_of::<Depth>(),
            )
        }
        .to_vec(),
    );
    additional_data.append(
        &mut unsafe {
            ::core::slice::from_raw_parts(
                (&Visual {
                    visual_id: 1,
                    class: 4,
                    bits_per_rgb_val: 32,
                    colormap_entries: 256,
                    red_mask: 0xFF,
                    green_mask: 0xFF00,
                    blue_mask: 0xFF0000,
                    pad0: 0,
                } as *const Visual) as *const u8,
                ::core::mem::size_of::<Visual>(),
            )
        }
        .to_vec(),
    );
    let conn_setup_prefix = ConnSetupPrefix {
        success: 1,
        length_reason: 0,
        major: client_prefix.major,
        minor: client_prefix.minor,
        additional_length: additional_data.len() as u16 / 4,
    };
    stream
        .write(&[
            conn_setup_prefix.success,
            conn_setup_prefix.length_reason,
            conn_setup_prefix.major.to_le_bytes()[0],
            conn_setup_prefix.major.to_le_bytes()[1],
            conn_setup_prefix.minor.to_le_bytes()[0],
            conn_setup_prefix.minor.to_le_bytes()[1],
            conn_setup_prefix.additional_length.to_le_bytes()[0],
            conn_setup_prefix.additional_length.to_le_bytes()[1],
        ])
        .ok();

    stream.write(&additional_data).ok()?;
    Some(Connection {
        endianness,
        sequence_number: 0,
    })
}
