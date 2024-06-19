use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use xaugh::connection::establish_connection;

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
        let request = connection.read_request(&stream)?;
        println!("{request:#?}");
        connection.write_response(&stream, request);
    }
}
