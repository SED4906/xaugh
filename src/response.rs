use std::{io::Write, net::TcpStream};

use crate::{request::Request, Connection};

impl Connection {
    pub fn empty_response(&mut self, mut stream: &TcpStream, extra_length: u32) {
        println!("(not really implemented)");
        let mut bytes_to_write = vec![
            1,
            0,
            self.to_bytes_16(self.sequence_number)[0],
            self.to_bytes_16(self.sequence_number)[1],
            self.to_bytes_32(extra_length)[0],
            self.to_bytes_32(extra_length)[1],
            self.to_bytes_32(extra_length)[2],
            self.to_bytes_32(extra_length)[3],
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

    pub fn write_response(&mut self, stream: &TcpStream, request: Request) {
        self.sequence_number += 1;
        match request {
            Request::CreateWindow { .. } => {}
            Request::ChangeWindowAttributes { .. } => {}
            Request::GetWindowAttributes { .. } => {
                self.empty_response(stream, 0);
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
                self.empty_response(stream, 0);
            }
            Request::QueryTree { .. } => {
                self.empty_response(stream, 0);
            }
            Request::InternAtom { .. } => {
                self.empty_response(stream, 0);
            }
            Request::GetAtomName { .. } => {
                self.empty_response(stream, 0);
            }
            Request::ChangeProperty { .. } => {}
            Request::DeleteProperty { .. } => {}
            Request::GetProperty { .. } => {
                self.empty_response(stream, 0);
            }
            Request::GetSelectionOwner { .. } => {
                self.empty_response(stream, 0);
            }
            Request::GrabServer => {}
            Request::GetInputFocus => {
                self.empty_response(stream, 0);
            }
            Request::OpenFont { .. } => {}
            Request::QueryFont { .. } => {
                self.empty_response(stream, 0);
            }
            Request::ListFonts { .. } => {
                self.empty_response(stream, 0);
            }
            Request::CreatePixmap { .. } => {}
            Request::FreePixmap { .. } => {}
            Request::CreateGC { .. } => {}
            Request::FreeGC { .. } => {}
            Request::PutImage { .. } => {}
            Request::QueryExtension { .. } => {
                self.empty_response(stream, 0);
            }
            Request::GetKeyboardControl => {
                self.empty_response(stream, 5);
            }
            Request::NoOperation => {}
            _ => todo!("response"),
        }
    }
}
