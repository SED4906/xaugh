use std::io::{Read, Write};

use crate::{connection::Connection, event::Event};

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
        only_if_exists: bool,
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
    RotateProperties {
        window: u32,
        delta: i16,
        properties: Vec<u32>,
    },
    ListProperties {
        window: u32,
    },
    SetSelectionOwner {
        owner: u32,
        selection: u32,
        time: u32,
    },
    GetSelectionOwner {
        selection: u32,
    },
    ConvertSelection {
        requestor: u32,
        selection: u32,
        target: u32,
        property: u32,
        time: u32,
    },
    SendEvent {
        propagate: bool,
        destination: u32,
        event_mask: u32,
        event: Event,
    },
    GrabPointer {
        owner_events: bool,
        grab_window: u32,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: u32,
        cursor: u32,
        time: u32,
    },
    UngrabPointer {
        time: u32,
    },
    GrabButton {
        owner_events: bool,
        grab_window: u32,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: u32,
        cursor: u32,
        button: u8,
        modifiers: u16,
    },
    UngrabButton {
        button: u8,
        grab_window: u32,
        modifiers: u16,
    },
    ChangeActivePointerGrab {
        cursor: u32,
        time: u32,
        event_mask: u16,
    },
    GrabKeyboard {
        owner_events: bool,
        grab_window: u32,
        time: u32,
        pointer_mode: u8,
        keyboard_mode: u8,
    },
    UngrabKeyboard {
        time: u32,
    },
    GrabKey {
        owner_events: bool,
        grab_window: u32,
        modifiers: u16,
        key: u8,
        pointer_mode: u8,
        keyboard_mode: u8,
    },
    UngrabKey {
        key: u8,
        grab_window: u32,
        modifiers: u16,
    },
    AllowEvents {
        mode: u8,
        time: u32,
    },
    GrabServer,
    UngrabServer,
    QueryPointer {
        window: u32,
    },
    GetMotionEvents {
        window: u32,
        start: u32,
        stop: u32,
    },
    TranslateCoordinates {
        src_window: u32,
        dst_window: u32,
        src_x: i16,
        src_y: i16,
    },
    WarpPointer {
        src_window: u32,
        dst_window: u32,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
    },
    SetInputFocus {
        revert_to: u8,
        focus: u32,
        time: u32,
    },
    GetInputFocus,
    QueryKeymap,
    OpenFont {
        fid: u32,
        name: String,
    },
    CloseFont {
        font: u32,
    },
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

impl<T: Read + Write> Connection<T> {
    pub fn read_request(&mut self) -> Option<Request> {
        let request_prefix = {
            let mut request_prefix_bytes = [0u8; 4];
            self.stream.read(&mut request_prefix_bytes).unwrap();
            RequestPrefix {
                opcode: request_prefix_bytes[0],
                extra: request_prefix_bytes[1],
                request_length: self.card16(&request_prefix_bytes[2..]),
            }
        };
        let mut request_bytes =
            vec![0; (request_prefix.request_length as usize).saturating_sub(1) * 4];
        self.stream.read(&mut request_bytes).unwrap();
        Some(match request_prefix.opcode {
            0 => {
                return None;
            }
            1 => {
                let window = self.card32(&request_bytes);
                let parent = self.card32(&request_bytes[4..]);
                let x = self.card16(&request_bytes[8..]);
                let y = self.card16(&request_bytes[10..]);
                let width = self.card16(&request_bytes[12..]);
                let height = self.card16(&request_bytes[14..]);
                let border_width = self.card16(&request_bytes[16..]);
                let class = self.card16(&request_bytes[18..]);
                let visual = self.card32(&request_bytes[20..]);
                let value_mask = self.card32(&request_bytes[24..]);
                let value_list = &self.copy8to32(&request_bytes[28..]);
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
                let window = self.card32(&request_bytes);
                let value_mask = self.card32(&request_bytes[4..]);
                let value_list = &self.copy8to32(&request_bytes[8..]);
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
                let window = self.card32(&request_bytes);
                Request::GetWindowAttributes { window }
            }
            4 => {
                let window = self.card32(&request_bytes);
                Request::DestroyWindow { window }
            }
            5 => Request::DestroySubwindows {
                window: self.card32(&request_bytes),
            },
            6 => Request::ChangeSaveSet {
                mode: request_prefix.extra,
                window: self.card32(&request_bytes),
            },
            7 => Request::ReparentWindow {
                window: self.card32(&request_bytes),
                parent: self.card32(&request_bytes[4..]),
                x: self.card16(&request_bytes[6..]),
                y: self.card16(&request_bytes[8..]),
            },
            8 => Request::MapWindow {
                window: self.card32(&request_bytes),
            },
            9 => Request::MapSubwindows {
                window: self.card32(&request_bytes),
            },
            10 => Request::UnmapWindow {
                window: self.card32(&request_bytes),
            },
            11 => Request::UnmapSubwindows {
                window: self.card32(&request_bytes),
            },
            12 => {
                let window = self.card32(&request_bytes);
                let value_mask = self.card16(&request_bytes[4..]);
                let value_list = &self.copy8to32(&request_bytes[8..]);
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
                window: self.card32(&request_bytes),
            },
            14 => Request::GetGeometry {
                drawable: self.card32(&request_bytes),
            },
            15 => Request::QueryTree {
                window: self.card32(&request_bytes),
            },
            16 => Request::InternAtom {
                only_if_exists: request_prefix.extra != 0,
                name: String::from_utf8_lossy(
                    &request_bytes[4..self.card16(&request_bytes) as usize + 4],
                )
                .to_string(),
            },
            17 => Request::GetAtomName {
                atom: self.card32(&request_bytes),
            },
            18 => {
                let mut data = request_bytes[20..].to_vec();
                data.truncate(
                    (request_bytes[12] as usize) * self.card32(&request_bytes[16..]) as usize,
                );
                Request::ChangeProperty {
                    mode: request_prefix.extra,
                    window: self.card32(&request_bytes),
                    property: self.card32(&request_bytes[4..]),
                    ptype: self.card32(&request_bytes[8..]),
                    format: request_bytes[12],
                    data,
                }
            }
            19 => Request::DeleteProperty {
                window: self.card32(&request_bytes),
                property: self.card32(&request_bytes[4..]),
            },
            20 => Request::GetProperty {
                delete: request_prefix.extra,
                window: self.card32(&request_bytes),
                property: self.card32(&request_bytes[4..]),
                typ: self.card32(&request_bytes[8..]),
                long_offset: self.card32(&request_bytes[12..]),
                long_length: self.card32(&request_bytes[16..]),
            },
            21 => Request::ListProperties {
                window: self.card32(&request_bytes),
            },
            22 => Request::SetSelectionOwner {
                owner: self.card32(&request_bytes),
                selection: self.card32(&request_bytes[4..]),
                time: self.card32(&request_bytes[8..]),
            },
            23 => Request::GetSelectionOwner {
                selection: self.card32(&request_bytes),
            },
            24 => Request::ConvertSelection {
                requestor: self.card32(&request_bytes),
                selection: self.card32(&request_bytes[4..]),
                target: self.card32(&request_bytes[8..]),
                property: self.card32(&request_bytes[12..]),
                time: self.card32(&request_bytes[16..]),
            },
            25 => Request::SendEvent {
                propagate: request_prefix.extra != 0,
                destination: self.card32(&request_bytes),
                event_mask: self.card32(&request_bytes[4..]),
                event: self.event(&request_bytes[8..]),
            },
            26 => Request::GrabPointer {
                owner_events: request_prefix.extra != 0,
                grab_window: self.card32(&request_bytes),
                event_mask: self.card16(&request_bytes[4..]),
                pointer_mode: request_bytes[6],
                keyboard_mode: request_bytes[7],
                confine_to: self.card32(&request_bytes[8..]),
                cursor: self.card32(&request_bytes[12..]),
                time: self.card32(&request_bytes[16..]),
            },
            27 => Request::UngrabPointer {
                time: self.card32(&request_bytes),
            },
            28 => Request::GrabButton {
                owner_events: request_prefix.extra != 0,
                grab_window: self.card32(&request_bytes),
                event_mask: self.card16(&request_bytes[4..]),
                pointer_mode: request_bytes[6],
                keyboard_mode: request_bytes[7],
                confine_to: self.card32(&request_bytes[8..]),
                cursor: self.card32(&request_bytes[12..]),
                button: request_bytes[14],
                modifiers: self.card16(&request_bytes[16..]),
            },
            29 => Request::UngrabButton {
                button: request_prefix.extra,
                grab_window: self.card32(&request_bytes),
                modifiers: self.card16(&request_bytes[4..]),
            },
            30 => Request::ChangeActivePointerGrab {
                cursor: self.card32(&request_bytes),
                time: self.card32(&request_bytes[4..]),
                event_mask: self.card16(&request_bytes[8..]),
            },
            31 => Request::GrabKeyboard {
                owner_events: request_prefix.extra != 0,
                grab_window: self.card32(&request_bytes),
                time: self.card32(&request_bytes[4..]),
                pointer_mode: request_bytes[8],
                keyboard_mode: request_bytes[9],
            },
            32 => Request::UngrabKeyboard {
                time: self.card32(&request_bytes),
            },
            33 => Request::GrabKey {
                owner_events: request_prefix.extra != 0,
                grab_window: self.card32(&request_bytes),
                modifiers: self.card16(&request_bytes[4..]),
                key: request_bytes[6],
                pointer_mode: request_bytes[7],
                keyboard_mode: request_bytes[8],
            },
            34 => Request::UngrabKey {
                key: request_prefix.extra,
                grab_window: self.card32(&request_bytes),
                modifiers: self.card16(&request_bytes[4..]),
            },
            35 => Request::AllowEvents {
                mode: request_prefix.extra,
                time: self.card32(&request_bytes),
            },
            36 => Request::GrabServer,
            37 => Request::UngrabServer,
            38 => Request::QueryPointer {
                window: self.card32(&request_bytes),
            },
            39 => Request::GetMotionEvents {
                window: self.card32(&request_bytes),
                start: self.card32(&request_bytes[4..]),
                stop: self.card32(&request_bytes[8..]),
            },
            40 => Request::TranslateCoordinates {
                src_window: self.card32(&request_bytes),
                dst_window: self.card32(&request_bytes[4..]),
                src_x: self.int16(&request_bytes[8..]),
                src_y: self.int16(&request_bytes[10..]),
            },
            41 => Request::WarpPointer {
                src_window: self.card32(&request_bytes),
                dst_window: self.card32(&request_bytes[4..]),
                src_x: self.int16(&request_bytes[8..]),
                src_y: self.int16(&request_bytes[10..]),
                src_width: self.card16(&request_bytes[12..]),
                src_height: self.card16(&request_bytes[14..]),
                dst_x: self.int16(&request_bytes[16..]),
                dst_y: self.int16(&request_bytes[18..]),
            },
            42 => Request::SetInputFocus {
                revert_to: request_prefix.extra,
                focus: self.card32(&request_bytes),
                time: self.card32(&request_bytes[4..]),
            },
            43 => Request::GetInputFocus,
            44 => Request::QueryKeymap,
            45 => Request::OpenFont {
                fid: self.card32(&request_bytes),
                name: String::from_utf8_lossy(
                    &request_bytes[8..self.card16(&request_bytes[4..]) as usize + 8],
                )
                .to_string(),
            },
            46 => Request::CloseFont {
                font: self.card32(&request_bytes),
            },
            47 => Request::QueryFont {
                fid: self.card32(&request_bytes),
            },
            49 => Request::ListFonts {
                max_names: self.card16(&request_bytes),
                pattern: String::from_utf8_lossy(
                    &request_bytes[4..self.card16(&request_bytes[2..]) as usize + 4],
                )
                .to_string(),
            },
            53 => Request::CreatePixmap {
                depth: request_prefix.extra,
                pid: self.card32(&request_bytes),
                drawable: self.card32(&request_bytes[4..]),
                width: self.card16(&request_bytes[8..]),
                height: self.card16(&request_bytes[10..]),
            },
            54 => Request::FreePixmap {
                pixmap: self.card32(&request_bytes),
            },
            55 => {
                let cid = self.card32(&request_bytes);
                let drawable = self.card32(&request_bytes[4..]);
                let value_mask = self.card32(&request_bytes[8..]);
                let mut value_list = [0u32; 23];
                let values = &self.copy8to32(&request_bytes[12..]);
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
                gc: self.card32(&request_bytes),
            },
            72 => Request::PutImage {
                format: request_prefix.extra,
                drawable: self.card32(&request_bytes),
                gc: self.card32(&request_bytes[4..]),
                width: self.card16(&request_bytes[8..]),
                height: self.card16(&request_bytes[10..]),
                dstx: self.int16(&request_bytes[12..]),
                dsty: self.int16(&request_bytes[14..]),
                leftpad: request_bytes[16],
                depth: request_bytes[17],
                data: request_bytes[20..].to_vec(),
            },
            98 => Request::QueryExtension {
                name: String::from_utf8_lossy(
                    &request_bytes[4..self.card16(&request_bytes) as usize + 4],
                )
                .to_string(),
            },
            103 => Request::GetKeyboardControl,
            114 => Request::RotateProperties {
                window: self.card32(&request_bytes),
                delta: self.int16(&request_bytes[6..]),
                properties: self.copy8to32(&request_bytes[8..]),
            },
            127 => Request::NoOperation,
            _ => todo!("{}", request_prefix.opcode),
        })
    }
}
