#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct PixmapFormat {
    pub depth: u8,
    pub bpp: u8,
    pub scanline_pad: u8,
    pub pad0: u8,
    pub pad1: u32,
}

pub static DEFAULT_PIXMAP_FORMATS: [PixmapFormat;1] = [PixmapFormat {depth: 1u8, bpp: 32u8, scanline_pad: 0u8, pad0: 0, pad1: 0 }];