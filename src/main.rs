use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use xaugh::{connection::establish_connection, request::read_request, response::write_response};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6001").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| match handle_connection(stream) {
            _ => {
                println!("Ended.")
            }
        });
    }
}

fn handle_connection(stream: TcpStream) -> Option<()> {
    let mut connection = establish_connection(&stream)?;
    loop {
        let request = read_request(&connection, &stream)?;
        println!("{request:#?}");
        write_response(&mut connection, &stream, request);
    }
}
