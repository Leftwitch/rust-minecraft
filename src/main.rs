#![allow(unused_must_use)]
mod protocol;

use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use protocol::ProxiedConnection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    for stream in listener.incoming() {
        if let Err(_stream) = stream {
            continue;
        }
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(stream: TcpStream) {
    let mut connection = ProxiedConnection::new(stream);
    connection.handle_connection();
}
