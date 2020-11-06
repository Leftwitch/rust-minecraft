use super::super::Packet;
use crate::protocol::{get_packet_size, read_varint, write_varint};
use std::{io::Read, io::Write, net::TcpStream};

pub struct MinecraftConnection {
    stream: TcpStream,
}

impl MinecraftConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream: stream }
    }
}
impl MinecraftConnection {
    pub fn send_packet<P: Packet>(&mut self, packet: &P) {
        let size: i32 = get_packet_size(packet) as i32;
        write_varint(&size, self);
        write_varint(&packet.get_id(), self);
        packet.write(&mut self.stream);
    }

    pub fn recv_packet(&mut self) -> i32 {
        let packet_length = read_varint(self);
        if let Ok(_packet_length) = packet_length {
            let packet_id = read_varint(self);

            if let Ok(packet_id) = packet_id {
                return packet_id;
            }
        };

        return -1;
    }

    pub fn get_stream(&self) -> &TcpStream {
        return &self.stream;
    }
}

impl Read for MinecraftConnection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Write for MinecraftConnection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}
