#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
mod connection;
mod pixmap;
mod request;
mod screen;

<<<<<<< HEAD
use std::{net::{TcpListener, TcpStream}, thread};
=======
use std::{
    net::{TcpListener, TcpStream},
    thread,
};
>>>>>>> b5703cb ((still incomplete))

use connection::{establish_connection, Connection};
use request::read_request;

use crate::request::respond_request;

static VENDOR: &str = "Xaugh X Server";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6001").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
<<<<<<< HEAD
        thread::spawn(|| {
            match handle_connection(stream) {
                Some(()) => println!("Succeeded."),
                None => println!("Failed.")
            }
=======
        thread::spawn(|| match handle_connection(stream) {
            Some(()) => println!("Succeeded."),
            None => println!("Failed."),
>>>>>>> b5703cb ((still incomplete))
        });
    }
}

fn handle_connection(stream: TcpStream) -> Option<()> {
<<<<<<< HEAD
    let _connection = establish_connection(&stream)?;
    loop {
        let request = read_request(&stream);
        println!("{request:#?}");
        respond_request(&stream, request);
        println!("responded");
    }
    Some(())
}
=======
    let mut connection = establish_connection(&stream)?;
    loop {
        let request = read_request(&stream);
        println!("{request:#?}");
        respond_request(&mut connection, &stream, request);
    }
    Some(())
}
>>>>>>> b5703cb ((still incomplete))
