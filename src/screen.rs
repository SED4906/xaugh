#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Screen {
    pub root_window: u32,
    pub default_colormap: u32,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_px: u16,
    pub height_px: u16,
    pub width_mm: u16,
    pub height_mm: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: u32,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub num_depths: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Depth {
    pub depth: u8,
    pub pad0: u8,
    pub number_of_visuals: u16,
    pub pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Visual {
    pub visual_id: u32,
    pub class: u8,
    pub bits_per_rgb_val: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad0: u32,
}
