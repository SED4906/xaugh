<<<<<<< HEAD
use std::{io::{Read, Write}, mem::size_of, net::TcpStream};

use xaugh::ReadStruct;
=======
use std::{
    io::{Read, Write},
    mem::size_of,
    net::TcpStream,
};

use xaugh::{card16, card32, ReadStruct};

use crate::connection::{self, Connection};
>>>>>>> b5703cb ((still incomplete))

#[repr(C)]
#[derive(Clone, Debug)]
pub enum Request {
    CreateWindow {
<<<<<<< HEAD
        base: CreateWindowRequestBase,
        value_list: [u32;15],
=======
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
>>>>>>> b5703cb ((still incomplete))
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
<<<<<<< HEAD
    InternAtom,
    GetAtomName,
    ChangeProperty,
    DeleteProperty,
    GetProperty,
=======
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
>>>>>>> b5703cb ((still incomplete))
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
    OpenFont,
    CloseFont,
    QueryFont,
    QueryTextExtents,
    ListFonts,
    ListFontsWithInfo,
    SetFontPath,
    GetFontPath,
<<<<<<< HEAD
    CreatePixmap,
    FreePixmap,
    CreateGC,
=======
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
>>>>>>> b5703cb ((still incomplete))
    ChangeGC,
    CopyGC,
    SetDashes,
    SetClipRectangles,
    FreeGC,
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
    PutImage,
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
<<<<<<< HEAD
    opcode: u8, 
=======
    opcode: u8,
>>>>>>> b5703cb ((still incomplete))
    extra: u8,
    request_length: u16,
}

<<<<<<< HEAD
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct CreateWindowRequestBase {
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
}

impl ReadStruct for RequestPrefix {}
impl ReadStruct for CreateWindowRequestBase {}

pub fn read_request(mut stream: &TcpStream) -> Request {
    let request_prefix = RequestPrefix::read_struct(stream);
    match request_prefix.opcode {
        1 => {
            let mut bytes = vec![0;(request_prefix.request_length as usize - 1) * 4];
            stream.read(&mut bytes).unwrap();
            let base = unsafe {
                *(core::ptr::from_ref(&bytes[..size_of::<CreateWindowRequestBase>()]) as *const CreateWindowRequestBase)
            };
            let values = unsafe {core::slice::from_raw_parts(core::ptr::from_ref(&bytes[size_of::<CreateWindowRequestBase>()..]) as *const u32,request_prefix.request_length as usize - 9)};
            let mut value_list = [0u32;15];
            for (index,value) in values.into_iter().enumerate() {
                let which = loop {
                    let mut times = 0;
                    let mut w = 0;
                    if base.value_mask & (1<<w) == 0 {
                        w+=1;
=======
impl ReadStruct for RequestPrefix {}

pub fn read_request(mut stream: &TcpStream) -> Request {
    let request_prefix = RequestPrefix::read_struct(stream);
    let mut request_bytes = vec![0; (request_prefix.request_length as usize).saturating_sub(1) * 4];
    stream.read(&mut request_bytes).unwrap();
    match request_prefix.opcode {
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
                let which = loop {
                    let mut times = 0;
                    let mut w = 0;
                    if value_mask & (1 << w) == 0 {
                        w += 1;
>>>>>>> b5703cb ((still incomplete))
                    } else if times < index {
                        times += 1;
                        w += 1;
                    } else {
                        break w;
                    }
                };
                value_list[which] = *value;
            }
<<<<<<< HEAD
            Request::CreateWindow {
                base: base,
                value_list: value_list,
            }
        },
        5 => {
            let mut bytes = [0u8;4];
            stream.read(&mut bytes).unwrap();
            let window = u32::from_le_bytes(bytes);
            Request::DestroySubwindows {
                window: window
            }
        }
        98 => {
            let mut bytes = vec![0;(request_prefix.request_length as usize - 1) * 4];
            stream.read(&mut bytes).unwrap();
            let n_name_len = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
            Request::QueryExtension {
                name: String::from_utf8_lossy(&bytes[4..n_name_len as usize+4]).to_string()
            }
        }
        _ => todo!("{}",request_prefix.opcode)
    }
}

pub fn respond_request(mut stream: &TcpStream, request: Request) {
    match request {
        Request::QueryExtension { name } => {
            stream.write(&[1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]).unwrap();
        }
        _ => {}
    }
}
=======
            Request::CreateWindow { wid, parent, x, y, width, height, border_width, class, visual, value_mask, value_list }
        }
        5 => {
            Request::DestroySubwindows { window: card32(&request_bytes) }
        }
        16 => {
            let n_name_len = (request_bytes[0] as u16) | ((request_bytes[1] as u16) << 8);
            Request::InternAtom {
                only_if_exists: request_prefix.extra,
                name: String::from_utf8_lossy(&request_bytes[4..n_name_len as usize + 4]).to_string(),
            }
        }
        20 => {
            Request::GetProperty { delete: request_prefix.extra,
                window: card32(&request_bytes), property: card32(&request_bytes[4..]), typ: card32(&request_bytes[8..]), long_offset: card32(&request_bytes[12..]), long_length: card32(&request_bytes[16..]) }
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
        98 => {
            let n_name_len = (request_bytes[0] as u16) | ((request_bytes[1] as u16) << 8);
            Request::QueryExtension {
                name: String::from_utf8_lossy(&request_bytes[4..n_name_len as usize + 4]).to_string(),
            }
        }
        _ => todo!("{}", request_prefix.opcode),
    }
}

pub fn respond_request_empty(connection: &mut Connection, mut stream: &TcpStream) {
    println!("(not really implemented)");
    stream
        .write(&[
            1, 0, connection.sequence_number.to_le_bytes()[0], connection.sequence_number.to_le_bytes()[1], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ])
        .unwrap();
}

pub fn respond_request(connection: &mut Connection, mut stream: &TcpStream, request: Request) {
    connection.sequence_number += 1;
    match request {
        Request::CreateWindow { .. } => {}
        Request::DestroySubwindows { .. } => {}
        Request::InternAtom { .. } => {
            respond_request_empty(connection, stream);
        }
        Request::GetProperty { .. } => {
            respond_request_empty(connection, stream);
        }
        Request::CreatePixmap { .. } => {}
        Request::FreePixmap { .. } => {}
        Request::CreateGC { .. } => {}
        Request::QueryExtension { name:_ } => {
            respond_request_empty(connection, stream);
        }
        _ => todo!("response")
    }
}
>>>>>>> b5703cb ((still incomplete))
