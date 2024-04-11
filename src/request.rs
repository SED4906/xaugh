use std::{
    io::{Read, Write},
    net::TcpStream,
};

use xaugh::{card16, card32, copy8to32, int16};

use crate::connection::{self, Connection};

#[repr(C)]
#[derive(Clone, Debug)]
pub enum Request {
    CreateWindow {
        wid: u32,
        parent: u32,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: u32,
        values: WindowAttributes,
        //value_mask: u32,
        //value_list: [u32; 15],
    },
    ChangeWindowAttributes {
        wid: u32,
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
    ChangeSaveSet,
    ReparentWindow,
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
    ConfigureWindow,
    CirculateWindow,
    GetGeometry {
        drawable: u32,
    },
    QueryTree,
    InternAtom {
        only_if_exists: u8,
        name: String
    },
    GetAtomName,
    ChangeProperty,
    DeleteProperty,
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
        value_list: [u32;23],
    },
    ChangeGC,
    CopyGC,
    SetDashes,
    SetClipRectangles,
    FreeGC {
        gc: u32
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
        data: Vec<u8>
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

pub fn read_request(mut stream: &TcpStream) -> Option<Request> {
    let mut request_prefix_bytes = [0u8;4];
    stream.read(&mut request_prefix_bytes).unwrap();
    let request_prefix = RequestPrefix {
        opcode: request_prefix_bytes[0],
        extra: request_prefix_bytes[1],
        request_length: card16(&request_prefix_bytes[2..]),
    };
    let mut request_bytes = vec![0; (request_prefix.request_length as usize).saturating_sub(1) * 4];
    stream.read(&mut request_bytes).unwrap();
    Some(match request_prefix.opcode {
        0 => {
            return None;
        }
        1 => {
            let wid = card32(&request_bytes);
            let parent = card32(&request_bytes[4..]);
            let x = card16(&request_bytes[8..]);
            let y = card16(&request_bytes[10..]);
            let width = card16(&request_bytes[12..]);
            let height = card16(&request_bytes[14..]);
            let border_width = card16(&request_bytes[16..]);
            let class = card16(&request_bytes[18..]);
            let visual = card32(&request_bytes[20..]);
            let value_mask = card32(&request_bytes[24..]);
            let value_list = &copy8to32(&request_bytes[28..]);
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
            Request::CreateWindow { wid, parent, x, y, width, height, border_width, class, visual, values }
        }
        2 => {
            let wid = card32(&request_bytes);
            let value_mask = card32(&request_bytes[4..]);
            let value_list = &copy8to32(&request_bytes[8..]);
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
            Request::ChangeWindowAttributes { wid, values }
        }
        3 => {
            let window = card32(&request_bytes);
            Request::GetWindowAttributes { window }
        }
        4 => {
            let window = card32(&request_bytes);
            Request::DestroyWindow { window }
        }
        5 => {
            Request::DestroySubwindows { window: card32(&request_bytes) }
        }
        8 => {
            Request::MapWindow { window: card32(&request_bytes) }
        }
        9 => {
            Request::MapSubwindows { window: card32(&request_bytes) }
        }
        10 => {
            Request::UnmapWindow { window: card32(&request_bytes) }
        }
        11 => {
            Request::UnmapSubwindows { window: card32(&request_bytes) }
        }
        14 => {
            Request::GetGeometry { drawable: card32(&request_bytes) }
        }
        16 => {
            Request::InternAtom {
                only_if_exists: request_prefix.extra,
                name: String::from_utf8_lossy(&request_bytes[4..card16(&request_bytes) as usize + 4]).to_string(),
            }
        }
        20 => {
            Request::GetProperty { delete: request_prefix.extra,
                window: card32(&request_bytes), property: card32(&request_bytes[4..]), typ: card32(&request_bytes[8..]), long_offset: card32(&request_bytes[12..]), long_length: card32(&request_bytes[16..]) }
        }
        23 => {
            Request::GetSelectionOwner { selection: card32(&request_bytes) }
        }
        36 => {
            Request::GrabServer
        }
        43 => {
            Request::GetInputFocus
        }
        45 => {
            Request::OpenFont { fid: card32(&request_bytes), name: String::from_utf8_lossy(&request_bytes[8..card16(&request_bytes[4..]) as usize + 8]).to_string() }
        }
        47 => {
            Request::QueryFont { fid: card32(&request_bytes) }
        }
        49 => {
            Request::ListFonts { max_names: card16(&request_bytes), pattern: String::from_utf8_lossy(&request_bytes[4..card16(&request_bytes[2..]) as usize + 4]).to_string() }
        }
        53 => {
            Request::CreatePixmap { depth: request_prefix.extra, pid: card32(&request_bytes), drawable: card32(&request_bytes[4..]), width: card16(&request_bytes[8..]), height: card16(&request_bytes[10..]) }
        }
        54 => {
            Request::FreePixmap { pixmap: card32(&request_bytes) }
        }
        55 => {
            let cid = card32(&request_bytes);
            let drawable = card32(&request_bytes[4..]);
            let value_mask = card32(&request_bytes[8..]);
            let mut value_list = [0u32; 23];
            let values =  &copy8to32(&request_bytes[12..]);
            for (index, value) in values.into_iter().enumerate() {
                let which = loop {
                    let mut times = 0;
                    let mut w = 0;
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
        60 => {
            Request::FreeGC { gc: card32(&request_bytes) }
        }
        72 => {
            Request::PutImage {
                format: request_prefix.extra,
                drawable: card32(&request_bytes),
                gc: card32(&request_bytes[4..]),
                width: card16(&request_bytes[8..]),
                height: card16(&request_bytes[10..]),
                dstx: int16(&request_bytes[12..]),
                dsty: int16(&request_bytes[14..]),
                leftpad: request_bytes[16],
                depth: request_bytes[17],
                data: request_bytes[20..].to_vec(),
            }
        }
        98 => {
            Request::QueryExtension {
                name: String::from_utf8_lossy(&request_bytes[4..card16(&request_bytes) as usize + 4]).to_string(),
            }
        }
        103 => Request::GetKeyboardControl,
        _ => todo!("{}", request_prefix.opcode),
    })
}

pub fn respond_request_empty(connection: &mut Connection, mut stream: &TcpStream, extra_length: u32) {
    println!("(not really implemented)");
    let mut bytes_to_write =vec![1, 0, connection.sequence_number.to_le_bytes()[0], connection.sequence_number.to_le_bytes()[1], extra_length.to_le_bytes()[0], extra_length.to_le_bytes()[1], extra_length.to_le_bytes()[2], extra_length.to_le_bytes()[3], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,];
    bytes_to_write.append(&mut vec![0;4*extra_length as usize]);
    stream
        .write(&bytes_to_write)
        .unwrap();
}

pub fn respond_request(connection: &mut Connection, stream: &TcpStream, request: Request) {
    connection.sequence_number += 1;
    match request {
        Request::CreateWindow { .. } => {}
        Request::ChangeWindowAttributes { .. } => {}
        Request::DestroySubwindows { .. } => {}
        Request::GetGeometry { .. } => {
            respond_request_empty(connection, stream, 0);
        }
        Request::InternAtom { .. } => {
            respond_request_empty(connection, stream, 0);
        }
        Request::GetProperty { .. } => {
            respond_request_empty(connection, stream, 0);
        }
        Request::GetSelectionOwner { .. } => {
            respond_request_empty(connection, stream, 0);
        }
        Request::GrabServer => {}
        Request::GetInputFocus => {
            respond_request_empty(connection, stream, 0);
        }
        Request::OpenFont { .. } => {}
        Request::QueryFont { .. } => {
            respond_request_empty(connection, stream, 0);
        }
        Request::ListFonts { .. } => {
            respond_request_empty(connection, stream, 0);
        }
        Request::GetWindowAttributes { .. } => {
            respond_request_empty(connection, stream, 0);
        }
        Request::CreatePixmap { .. } => {}
        Request::FreePixmap { .. } => {}
        Request::CreateGC { .. } => {}
        Request::FreeGC { .. } => {}
        Request::PutImage { .. } => {}
        Request::QueryExtension { .. } => {
            respond_request_empty(connection, stream, 0);
        }
        Request::GetKeyboardControl => {
            respond_request_empty(connection, stream, 5);
        }
        _ => todo!("response")
    }
}