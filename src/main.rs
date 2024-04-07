#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
mod connection;
mod pixmap;
mod request;
mod screen;

use std::{net::{TcpListener, TcpStream}, thread};

use connection::{establish_connection, Connection};
use request::read_request;

use crate::request::respond_request;

static VENDOR: &str = "Xaugh X Server";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6001").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            match handle_connection(stream) {
                Some(()) => println!("Succeeded."),
                None => println!("Failed.")
            }
        });
    }
}

fn handle_connection(stream: TcpStream) -> Option<()> {
    let _connection = establish_connection(&stream)?;
    loop {
        let request = read_request(&stream);
        println!("{request:#?}");
        respond_request(&stream, request);
        println!("responded");
    }
    Some(())
}