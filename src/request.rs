use std::{io::{Read, Write}, mem::size_of, net::TcpStream};

use xaugh::ReadStruct;

#[repr(C)]
#[derive(Clone, Debug)]
pub enum Request {
    CreateWindow {
        base: CreateWindowRequestBase,
        value_list: [u32;15],
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
    InternAtom,
    GetAtomName,
    ChangeProperty,
    DeleteProperty,
    GetProperty,
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
    CreatePixmap,
    FreePixmap,
    CreateGC,
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
    opcode: u8, 
    extra: u8,
    request_length: u16,
}

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
                    } else if times < index {
                        times += 1;
                        w += 1;
                    } else {
                        break w;
                    }
                };
                value_list[which] = *value;
            }
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