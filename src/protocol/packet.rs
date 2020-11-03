use super::write_varint;
use std::io::{Cursor, Read, Write};
pub trait Packet {
    fn read<R: Read>(&mut self, reader: &mut R);
    fn write<W: Write>(&self, writer: &mut W);
    fn get_id(&self) -> i32;
}

pub trait PacketReceiver {
    fn send_packet<P: Packet>(&mut self, packet: &P);
}

pub fn get_packet_size<T: Packet>(packet: &T) -> usize {
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);

    write_varint(&packet.get_id(), &mut cursor);
    packet.write(&mut cursor);

    return bytes.len();
}
