use std::io::{Read, Write};

use crate::{connection::Connection, request::Request};

impl<T: Read + Write> Connection<T> {
    pub fn stub_response(&mut self, extra_length: u32) {
        println!("(stubbed)");
        let mut bytes_to_write = self.empty_response(extra_length,0);
        bytes_to_write.append(&mut vec![0;24 + extra_length as usize * 4]);
        self.stream.write(&bytes_to_write).unwrap();
    }

    pub fn empty_response(&mut self, extra_length: u32, extra: u8) -> Vec<u8> {
        vec![
            1,
            extra,
            self.to_bytes_16(self.sequence_number)[0],
            self.to_bytes_16(self.sequence_number)[1],
            self.to_bytes_32(extra_length)[0],
            self.to_bytes_32(extra_length)[1],
            self.to_bytes_32(extra_length)[2],
            self.to_bytes_32(extra_length)[3],
        ]
    }

    pub fn write_response(&mut self, request: Request) {
        self.sequence_number += 1;
        match request {
            Request::CreateWindow { .. } => {}
            Request::ChangeWindowAttributes { .. } => {}
            Request::GetWindowAttributes { .. } => {
                self.stub_response(0);
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
                self.stub_response(0);
            }
            Request::QueryTree { .. } => {
                self.stub_response(0);
            }
            Request::InternAtom { only_if_exists, name } => {
                let atom = crate::atom::get_atom(only_if_exists, name);
                let mut bytes_to_write = self.empty_response(0, 0);
                bytes_to_write.append(&mut self.to_bytes_32(atom).to_vec());
                bytes_to_write.append(&mut vec![0;20]);
                self.stream.write(&bytes_to_write).unwrap();
            }
            Request::GetAtomName { .. } => {
                self.stub_response(0);
            }
            Request::ChangeProperty { .. } => {}
            Request::DeleteProperty { .. } => {}
            Request::GetProperty { .. } => {
                self.stub_response(0);
            }
            Request::GetSelectionOwner { .. } => {
                self.stub_response(0);
            }
            Request::GrabServer => {}
            Request::GetInputFocus => {
                self.stub_response(0);
            }
            Request::OpenFont { .. } => {}
            Request::QueryFont { .. } => {
                self.stub_response(0);
            }
            Request::ListFonts { .. } => {
                self.stub_response(0);
            }
            Request::CreatePixmap { .. } => {}
            Request::FreePixmap { .. } => {}
            Request::CreateGC { .. } => {}
            Request::FreeGC { .. } => {}
            Request::PutImage { .. } => {}
            Request::QueryExtension { .. } => {
                self.stub_response(0);
            }
            Request::GetKeyboardControl => {
                self.stub_response(5);
            }
            Request::NoOperation => {}
            _ => todo!("response"),
        }
    }
}
