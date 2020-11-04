#![allow(unused_must_use)]
mod protocol;
use byteorder::ReadBytesExt;
use protocol::*;
use std::{
    net::{TcpListener, TcpStream},
    thread,
};

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
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
            println!("Connection closed");
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut data: [u8; 2] = [0; 2];
        stream.peek(&mut data);

        if data[0] == 0xFE {
            println!("Legacy Ping");
            println!("Legacy ping data {}", data[1]);
            //Move Cursor forward
            stream.read_u8();
            stream.read_u8();
            return;
        }

        if let Ok(packet_prefix) = read_varint(&mut stream) {
            println!(
                "\n####### NEW PACKET ######## \nPacket Prefix: {}\n----------",
                packet_prefix
            );

            //Every other packet is prefixed with the length
            if let Ok(packet_id) = read_varint(&mut stream) {
                println!("Packet Id: {}", packet_id);

                if packet_id == 0 {
                    //Empty request packet

                    if packet_prefix > 1 {
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
                    } else {
                        println!("Request with empty fields");
                        //Construct server list packet and send

                        let packet = PacketServerList {};
                        stream.send_packet(&packet);
                    }
                }

                if packet_id == 1 {
                    let mut ping = PacketPingPong { challenge: None };
                    ping.read(&mut stream);
                    println!("Ping Packet {:?}", ping);

                    let pong = ping;
                    stream.send_packet(&pong);
                }
            }
        }
    }
}
