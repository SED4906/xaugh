use std::{io::Write, net::TcpStream};

use crate::{connection::Connection, request::Request};

pub fn empty_response(
    connection: &mut Connection,
    mut stream: &TcpStream,
    extra_length: u32,
) {
    println!("(not really implemented)");
    let mut bytes_to_write = vec![
        1,
        0,
        connection.sequence_number.to_le_bytes()[0],
        connection.sequence_number.to_le_bytes()[1],
        extra_length.to_le_bytes()[0],
        extra_length.to_le_bytes()[1],
        extra_length.to_le_bytes()[2],
        extra_length.to_le_bytes()[3],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    bytes_to_write.append(&mut vec![0; 4 * extra_length as usize]);
    stream.write(&bytes_to_write).unwrap();
}

pub fn write_response(connection: &mut Connection, stream: &TcpStream, request: Request) {
    connection.sequence_number += 1;
    match request {
        Request::CreateWindow { .. } => {}
        Request::ChangeWindowAttributes { .. } => {}
        Request::GetWindowAttributes { .. } => {
            empty_response(connection, stream, 0);
        }
        Request::DestroyWindow { .. } => {}
        Request::DestroySubwindows { .. } => {}
        Request::ChangeSaveSet { .. } => {}
        Request::ReparentWindow { .. } => {}
        Request::MapWindow { .. } => {}
        Request::MapSubwindows { .. } => {}
        Request::UnmapWindow { .. } => {}
        Request::UnmapSubwindows { .. } => {}
        Request::ConfigureWindow { .. } => {}
        Request::CirculateWindow { .. } => {}
        Request::GetGeometry { .. } => {
            empty_response(connection, stream, 0);
        }
        Request::QueryTree { .. } => {
            empty_response(connection, stream, 0);
        }
        Request::InternAtom { .. } => {
            empty_response(connection, stream, 0);
        }
        Request::GetAtomName { .. } => {
            empty_response(connection, stream, 0);
        }
        Request::ChangeProperty { .. } => {}
        Request::DeleteProperty { .. } => {}
        Request::GetProperty { .. } => {
            empty_response(connection, stream, 0);
        }
        Request::GetSelectionOwner { .. } => {
            empty_response(connection, stream, 0);
        }
        Request::GrabServer => {}
        Request::GetInputFocus => {
            empty_response(connection, stream, 0);
        }
        Request::OpenFont { .. } => {}
        Request::QueryFont { .. } => {
            empty_response(connection, stream, 0);
        }
        Request::ListFonts { .. } => {
            empty_response(connection, stream, 0);
        }
        Request::CreatePixmap { .. } => {}
        Request::FreePixmap { .. } => {}
        Request::CreateGC { .. } => {}
        Request::FreeGC { .. } => {}
        Request::PutImage { .. } => {}
        Request::QueryExtension { .. } => {
            empty_response(connection, stream, 0);
        }
        Request::GetKeyboardControl => {
            empty_response(connection, stream, 5);
        }
        Request::NoOperation => {}
        _ => todo!("response"),
    }
}
