use std::{
    io::{Read, Write},
    net::TcpStream,
};

use xaugh::{card16, card32, int16};

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
        value_mask: u32,
        value_list: [u32; 15],
    },
    ChangeWindowAttributes,
    GetWindowAttributes,
    DestroyWindow,
    DestroySubwindows {
        window: u32,
    },
    ChangeSaveSet,
    ReparentWindow,
    MapWindow,
    MapSubwindows,
    UnmapWindow,
    UnmapSubwindows,
    ConfigureWindow,
    CirculateWindow,
    GetGeometry,
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
    GetSelectionOwner,
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
#[derive(Clone, Copy, Debug)]
pub struct RequestPrefix {
    opcode: u8, 
    extra: u8,
    request_length: u16,
}

pub fn read_request(mut stream: &TcpStream) -> Request {
    let mut request_prefix_bytes = [0u8;4];
    stream.read(&mut request_prefix_bytes).unwrap();
    let request_prefix = RequestPrefix {
        opcode: request_prefix_bytes[0],
        extra: request_prefix_bytes[1],
        request_length: card16(&request_prefix_bytes[2..]),
    };
    let mut request_bytes = vec![0; (request_prefix.request_length as usize).saturating_sub(1) * 4];
    stream.read(&mut request_bytes).unwrap();
    match request_prefix.opcode {
        0 => {
            println!("remaining bytes when died of death:");
            let mut buf = vec![];
            stream.read_to_end(&mut buf).ok();
            println!("{request_prefix_bytes:#?} {request_bytes:#?} {:#?}", buf);
            panic!("augh");
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
            let values = unsafe {
                core::slice::from_raw_parts(
                    core::ptr::from_ref(&request_bytes[28..])
                        as *const u32,
                    request_prefix.request_length as usize - 9,
                )
            };
            let mut value_list = [0u32; 15];
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
            Request::CreateWindow { wid, parent, x, y, width, height, border_width, class, visual, value_mask, value_list }
        }
        5 => {
            Request::DestroySubwindows { window: card32(&request_bytes) }
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
            let values = unsafe {
                core::slice::from_raw_parts(
                    core::ptr::from_ref(&request_bytes[12..])
                        as *const u32,
                    request_prefix.request_length as usize - 4,
                )
            };
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
        _ => todo!("{}", request_prefix.opcode),
    }
}

pub fn respond_request_empty(connection: &mut Connection, mut stream: &TcpStream, empty_following: usize) {
    println!("(not really implemented)");
    let mut bytes = vec![
        1, 0, connection.sequence_number.to_le_bytes()[0], connection.sequence_number.to_le_bytes()[1], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
    if empty_following > 0 {bytes.append(&mut vec![0u8;empty_following*4])};
    stream
        .write(&bytes)
        .unwrap();
}

pub fn respond_request(connection: &mut Connection, stream: &TcpStream, request: Request) {
    connection.sequence_number += 1;
    match request {
        Request::CreateWindow { .. } => {}
        Request::DestroySubwindows { .. } => {}
        Request::InternAtom { .. } => {
            respond_request_empty(connection, stream, 0);
        }
        Request::GetProperty { .. } => {
            respond_request_empty(connection, stream, 0);
        }
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
        Request::CreatePixmap { .. } => {}
        Request::FreePixmap { .. } => {}
        Request::CreateGC { .. } => {}
        Request::FreeGC { .. } => {}
        Request::PutImage { .. } => {}
        Request::QueryExtension { name:_ } => {
            respond_request_empty(connection, stream, 0);
        }
        _ => todo!("response")
    }
}