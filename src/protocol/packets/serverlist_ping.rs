use std::io::{Read, Write};

use super::super::*;

pub struct PacketServerList {}
impl Packet for PacketServerList {
    fn read<R: Read>(&mut self, _reader: &mut R) {
        todo!()
    }

    fn write<W: Write>(&self, writer: &mut W) {
        write_String("{ \"version\": { \"name\": \"1.16.3\", \"protocol\": 753 }, \"players\": { \"max\": 100, \"online\": 5, \"sample\": [ { \"name\": \"thinkofdeath\", \"id\": \"4566e69f-c907-48ee-8d71-d7ba5aa00d20\" } ] }, \"description\": { \"text\": \"UH IH UH AH AH TING TANG\" }}", writer);
    }

    fn get_id(&self) -> i32 {
        0
    }
}
