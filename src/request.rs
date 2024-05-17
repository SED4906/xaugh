use std::{
    io::Read,
    net::TcpStream,
};

use crate::{card16, card32, copy8to32, int16};

use crate::connection::Connection;

#[repr(C)]
#[derive(Clone, Debug)]
pub enum Request {
    CreateWindow {
        window: u32,
        parent: u32,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: u32,
        values: WindowAttributes,
    },
    ChangeWindowAttributes {
        window: u32,
        values: WindowAttributes,
    },
    GetWindowAttributes {
        window: u32,
    },
    DestroyWindow {
        window: u32,
    },
    DestroySubwindows {
        window: u32,
    },
    ChangeSaveSet {
        mode: u8,
        window: u32,
    },
    ReparentWindow {
        window: u32,
        parent: u32,
        x: u16,
        y: u16,
    },
    MapWindow {
        window: u32,
    },
    MapSubwindows {
        window: u32,
    },
    UnmapWindow {
        window: u32,
    },
    UnmapSubwindows {
        window: u32,
    },
    ConfigureWindow {
        window: u32,
        values: ConfigureValues,
    },
    CirculateWindow {
        direction: u8,
        window: u32,
    },
    GetGeometry {
        drawable: u32,
    },
    QueryTree {
        window: u32,
    },
    InternAtom {
        only_if_exists: u8,
        name: String,
    },
    GetAtomName {
        atom: u32,
    },
    ChangeProperty {
        mode: u8,
        window: u32,
        property: u32,
        ptype: u32,
        format: u8,
        data: Vec<u8>,
    },
    DeleteProperty {
        window: u32,
        property: u32,
    },
    GetProperty {
        delete: u8,
        window: u32,
        property: u32,
        typ: u32,
        long_offset: u32,
        long_length: u32,
    },
    RotateProperties,
    ListProperties,
    SetSelectionOwner,
    GetSelectionOwner {
        selection: u32,
    },
    ConvertSelection,
    SendEvent,
    GrabPointer,
    UngrabPointer,
    GrabButton,
    UngrabButton,
    ChangeActivePointerGrab,
    GrabKeyboard,
    UngrabKeyboard,
    GrabKey,
    UngrabKey,
    AllowEvents,
    GrabServer,
    UngrabServer,
    QueryPointer,
    GetMotionEvents,
    TranslateCoordinates,
    WarpPointer,
    SetInputFocus,
    GetInputFocus,
    QueryKeymap,
    OpenFont {
        fid: u32,
        name: String,
    },
    CloseFont,
    QueryFont {
        fid: u32,
    },
    QueryTextExtents,
    ListFonts {
        max_names: u16,
        pattern: String,
    },
    ListFontsWithInfo,
    SetFontPath,
    GetFontPath,
    CreatePixmap {
        depth: u8,
        pid: u32,
        drawable: u32,
        width: u16,
        height: u16,
    },
    FreePixmap {
        pixmap: u32,
    },
    CreateGC {
        cid: u32,
        drawable: u32,
        value_mask: u32,
        value_list: [u32; 23],
    },
    ChangeGC,
    CopyGC,
    SetDashes,
    SetClipRectangles,
    FreeGC {
        gc: u32,
    },
    ClearArea,
    CopyArea,
    CopyPlane,
    PolyPoint,
    PolyLine,
    PolySegment,
    PolyRectangle,
    PolyArc,
    FillPoly,
    PolyFillRectangle,
    PolyFillArc,
    PutImage {
        format: u8,
        drawable: u32,
        gc: u32,
        width: u16,
        height: u16,
        dstx: i16,
        dsty: i16,
        leftpad: u8,
        depth: u8,
        data: Vec<u8>,
    },
    GetImage,
    PolyText8,
    PolyText16,
    ImageText8,
    ImageText16,
    CreateColormap,
    FreeColormap,
    CopyColormapAndFree,
    InstallColormap,
    UninstallColormap,
    ListInstalledColormaps,
    AllocColor,
    AllocNamedColor,
    AllocColorCells,
    AllocColorPlanes,
    FreeColors,
    StoreColors,
    StoreNamedColor,
    QueryColors,
    LookupColor,
    CreateCursor,
    CreateGlyphCursor,
    FreeCursor,
    RecolorCursor,
    QueryBestSize,
    QueryExtension {
        name: String,
    },
    ListExtensions,
    SetModifierMapping,
    GetModifierMapping,
    ChangeKeyboardMapping,
    GetKeyboardMapping,
    ChangeKeyboardControl,
    GetKeyboardControl,
    Bell,
    SetPointerMapping,
    GetPointerMapping,
    ChangePointerControl,
    GetPointerControl,
    SetScreenSaver,
    GetScreenSaver,
    ForceScreenSaver,
    ChangeHosts,
    ListHosts,
    SetAccessControl,
    SetCloseDownMode,
    KillClient,
    NoOperation,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConfigureValues {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    border_width: u32,
    sibling: u32,
    stack_mode: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WindowAttributes {
    background_pixmap: u32,
    background_pixel: u32,
    border_pixmap: u32,
    border_pixel: u32,
    border_gravity: u32,
    win_gravity: u32,
    backing_store: u32,
    backing_planes: u32,
    backing_pixel: u32,
    override_redirect: u32,
    save_under: u32,
    event_mask: u32,
    do_not_propogate_mask: u32,
    colormap: u32,
    cursor: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RequestPrefix {
    opcode: u8,
    extra: u8,
    request_length: u16,
}

pub fn read_request(connection: &Connection, mut stream: &TcpStream) -> Option<Request> {
    let mut request_prefix_bytes = [0u8; 4];
    stream.read(&mut request_prefix_bytes).unwrap();
    let request_prefix = RequestPrefix {
        opcode: request_prefix_bytes[0],
        extra: request_prefix_bytes[1],
        request_length: card16(connection, &request_prefix_bytes[2..]),
    };
    let mut request_bytes = vec![0; (request_prefix.request_length as usize).saturating_sub(1) * 4];
    stream.read(&mut request_bytes).unwrap();
    Some(match request_prefix.opcode {
        0 => {
            return None;
        }
        1 => {
            let window = card32(connection, &request_bytes);
            let parent = card32(connection, &request_bytes[4..]);
            let x = card16(connection, &request_bytes[8..]);
            let y = card16(connection, &request_bytes[10..]);
            let width = card16(connection, &request_bytes[12..]);
            let height = card16(connection, &request_bytes[14..]);
            let border_width = card16(connection, &request_bytes[16..]);
            let class = card16(connection, &request_bytes[18..]);
            let visual = card32(connection, &request_bytes[20..]);
            let value_mask = card32(connection, &request_bytes[24..]);
            let value_list = &copy8to32(connection, &request_bytes[28..]);
            let mut values = WindowAttributes::default();
            for (index, value) in value_list.into_iter().enumerate() {
                let mut times = 0;
                let mut w = 0;
                let which = loop {
                    if w >= 32 {
                        panic!("bit mask error");
                    }
                    if value_mask & (1 << w) == 0 {
                        w += 1;
                    } else if times < index {
                        times += 1;
                        w += 1;
                    } else {
                        break w;
                    }
                };
                match which {
                    0 => values.background_pixmap = *value,
                    1 => values.background_pixel = *value,
                    2 => values.border_pixmap = *value,
                    3 => values.border_pixel = *value,
                    4 => values.border_gravity = *value,
                    5 => values.win_gravity = *value,
                    6 => values.backing_store = *value,
                    7 => values.backing_planes = *value,
                    8 => values.backing_pixel = *value,
                    9 => values.override_redirect = *value,
                    10 => values.save_under = *value,
                    11 => values.event_mask = *value,
                    12 => values.do_not_propogate_mask = *value,
                    13 => values.colormap = *value,
                    14 => values.cursor = *value,
                    _ => panic!("invalid bit in window attribute mask"),
                }
            }
            Request::CreateWindow {
                window,
                parent,
                x,
                y,
                width,
                height,
                border_width,
                class,
                visual,
                values,
            }
        }
        2 => {
            let window = card32(connection, &request_bytes);
            let value_mask = card32(connection, &request_bytes[4..]);
            let value_list = &copy8to32(connection, &request_bytes[8..]);
            let mut values = WindowAttributes::default();
            for (index, value) in value_list.into_iter().enumerate() {
                let mut times = 0;
                let mut w = 0;
                let which = loop {
                    if w >= 32 {
                        panic!("bit mask error");
                    }
                    if value_mask & (1 << w) == 0 {
                        w += 1;
                    } else if times < index {
                        times += 1;
                        w += 1;
                    } else {
                        break w;
                    }
                };
                match which {
                    0 => values.background_pixmap = *value,
                    1 => values.background_pixel = *value,
                    2 => values.border_pixmap = *value,
                    3 => values.border_pixel = *value,
                    4 => values.border_gravity = *value,
                    5 => values.win_gravity = *value,
                    6 => values.backing_store = *value,
                    7 => values.backing_planes = *value,
                    8 => values.backing_pixel = *value,
                    9 => values.override_redirect = *value,
                    10 => values.save_under = *value,
                    11 => values.event_mask = *value,
                    12 => values.do_not_propogate_mask = *value,
                    13 => values.colormap = *value,
                    14 => values.cursor = *value,
                    _ => panic!("invalid bit in window attribute mask"),
                }
            }
            Request::ChangeWindowAttributes { window, values }
        }
        3 => {
            let window = card32(connection, &request_bytes);
            Request::GetWindowAttributes { window }
        }
        4 => {
            let window = card32(connection, &request_bytes);
            Request::DestroyWindow { window }
        }
        5 => Request::DestroySubwindows {
            window: card32(connection, &request_bytes),
        },
        6 => Request::ChangeSaveSet {
            mode: request_prefix.extra,
            window: card32(connection, &request_bytes),
        },
        7 => Request::ReparentWindow {
            window: card32(connection, &request_bytes),
            parent: card32(connection, &request_bytes[4..]),
            x: card16(connection, &request_bytes[6..]),
            y: card16(connection, &request_bytes[8..]),
        },
        8 => Request::MapWindow {
            window: card32(connection, &request_bytes),
        },
        9 => Request::MapSubwindows {
            window: card32(connection, &request_bytes),
        },
        10 => Request::UnmapWindow {
            window: card32(connection, &request_bytes),
        },
        11 => Request::UnmapSubwindows {
            window: card32(connection, &request_bytes),
        },
        12 => {
            let window = card32(connection, &request_bytes);
            let value_mask = card16(connection, &request_bytes[4..]);
            let value_list = &copy8to32(connection, &request_bytes[8..]);
            let mut values = ConfigureValues::default();
            for (index, value) in value_list.into_iter().enumerate() {
                let mut times = 0;
                let mut w = 0;
                let which = loop {
                    if w >= 16 {
                        panic!("bit mask error");
                    }
                    if value_mask & (1 << w) == 0 {
                        w += 1;
                    } else if times < index {
                        times += 1;
                        w += 1;
                    } else {
                        break w;
                    }
                };
                match which {
                    0 => values.x = *value,
                    1 => values.y = *value,
                    2 => values.width = *value,
                    3 => values.height = *value,
                    4 => values.border_width = *value,
                    5 => values.sibling = *value,
                    6 => values.stack_mode = *value,
                    _ => panic!("invalid bit in window attribute mask"),
                }
            }
            Request::ConfigureWindow { window, values }
        }
        13 => Request::CirculateWindow {
            direction: request_prefix.extra,
            window: card32(connection, &request_bytes),
        },
        14 => Request::GetGeometry {
            drawable: card32(connection, &request_bytes),
        },
        15 => Request::QueryTree {
            window: card32(connection, &request_bytes),
        },
        16 => Request::InternAtom {
            only_if_exists: request_prefix.extra,
            name: String::from_utf8_lossy(
                &request_bytes[4..card16(connection, &request_bytes) as usize + 4],
            )
            .to_string(),
        },
        17 => Request::GetAtomName {
            atom: card32(connection, &request_bytes),
        },
        18 => {
            let mut data = request_bytes[20..].to_vec();
            data.truncate(
                (request_bytes[12] as usize) * card32(connection, &request_bytes[16..]) as usize,
            );
            Request::ChangeProperty {
                mode: request_prefix.extra,
                window: card32(connection, &request_bytes),
                property: card32(connection, &request_bytes[4..]),
                ptype: card32(connection, &request_bytes[8..]),
                format: request_bytes[12],
                data,
            }
        }
        19 => Request::DeleteProperty {
            window: card32(connection, &request_bytes),
            property: card32(connection, &request_bytes[4..]),
        },
        20 => Request::GetProperty {
            delete: request_prefix.extra,
            window: card32(connection, &request_bytes),
            property: card32(connection, &request_bytes[4..]),
            typ: card32(connection, &request_bytes[8..]),
            long_offset: card32(connection, &request_bytes[12..]),
            long_length: card32(connection, &request_bytes[16..]),
        },
        23 => Request::GetSelectionOwner {
            selection: card32(connection, &request_bytes),
        },
        36 => Request::GrabServer,
        43 => Request::GetInputFocus,
        45 => Request::OpenFont {
            fid: card32(connection, &request_bytes),
            name: String::from_utf8_lossy(
                &request_bytes[8..card16(connection, &request_bytes[4..]) as usize + 8],
            )
            .to_string(),
        },
        47 => Request::QueryFont {
            fid: card32(connection, &request_bytes),
        },
        49 => Request::ListFonts {
            max_names: card16(connection, &request_bytes),
            pattern: String::from_utf8_lossy(
                &request_bytes[4..card16(connection, &request_bytes[2..]) as usize + 4],
            )
            .to_string(),
        },
        53 => Request::CreatePixmap {
            depth: request_prefix.extra,
            pid: card32(connection, &request_bytes),
            drawable: card32(connection, &request_bytes[4..]),
            width: card16(connection, &request_bytes[8..]),
            height: card16(connection, &request_bytes[10..]),
        },
        54 => Request::FreePixmap {
            pixmap: card32(connection, &request_bytes),
        },
        55 => {
            let cid = card32(connection, &request_bytes);
            let drawable = card32(connection, &request_bytes[4..]);
            let value_mask = card32(connection, &request_bytes[8..]);
            let mut value_list = [0u32; 23];
            let values = &copy8to32(connection, &request_bytes[12..]);
            for (index, value) in values.into_iter().enumerate() {
                let mut times = 0;
                let mut w = 0;
                let which = loop {
                    if value_mask & (1 << w) == 0 {
                        w += 1;
                    } else if times < index {
                        times += 1;
                        w += 1;
                    } else {
                        break w;
                    }
                };
                value_list[which] = *value;
            }
            Request::CreateGC {
                cid,
                drawable,
                value_mask,
                value_list,
            }
        }
        60 => Request::FreeGC {
            gc: card32(connection, &request_bytes),
        },
        72 => Request::PutImage {
            format: request_prefix.extra,
            drawable: card32(connection, &request_bytes),
            gc: card32(connection, &request_bytes[4..]),
            width: card16(connection, &request_bytes[8..]),
            height: card16(connection, &request_bytes[10..]),
            dstx: int16(connection, &request_bytes[12..]),
            dsty: int16(connection, &request_bytes[14..]),
            leftpad: request_bytes[16],
            depth: request_bytes[17],
            data: request_bytes[20..].to_vec(),
        },
        98 => Request::QueryExtension {
            name: String::from_utf8_lossy(
                &request_bytes[4..card16(connection, &request_bytes) as usize + 4],
            )
            .to_string(),
        },
        103 => Request::GetKeyboardControl,
        127 => Request::NoOperation,
        _ => todo!("{}", request_prefix.opcode),
    })
}
