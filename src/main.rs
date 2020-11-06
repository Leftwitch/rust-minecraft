#![allow(unused_must_use)]
mod protocol;
use protocol::*;
use std::{
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    for stream in listener.incoming() {
        if let Err(_stream) = stream {
            continue;
        }
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
            println!("Connection closed");
        });
    }
}

fn handle_connection(stream: TcpStream) {
    let mut connection = UserConnection::new(stream);
    loop {
        connection.recv_packet();
    }
}
