#![allow(unused_must_use)]
mod protocol;
use protocol::*;
use std::net::{TcpListener, TcpStream};

impl PacketReceiver for TcpStream {
    fn send_packet<P: Packet>(&mut self, packet: &P) {
        let size: i32 = get_packet_size(packet) as i32;
        write_varint(&size, self);
        write_varint(&packet.get_id(), self);
        packet.write(self);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_connection(&mut stream);
    }
}

fn handle_connection(mut stream: &mut TcpStream) {
    if let Ok(packet_size) = read_varint(&mut stream) {
        println!("Packet Size: {}", packet_size);
        if let Ok(packet_id) = read_varint(stream) {
            println!("Packet Id: {}", packet_id);

            //If Packet is Handshake
            if packet_id == 0 {
                //Create Handshake instanze TODO
                let mut handshake = PacketHandshake {
                    protocol_version: None,
                    server_address: None,
                    port: None,
                    next_state: None,
                };
                //Read Packet
                handshake.read(&mut stream);

                //Output Packet
                println!("{:?}", handshake);

                //Construct server list packet and send
                let packet = PacketServerList {};
                stream.send_packet(&packet);
            }
        }
    }
}
