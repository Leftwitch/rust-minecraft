use std::io::{Read, Write};

use super::super::*;

#[derive(Debug)]
pub struct PacketSetCompression {
    pub threshhold: Option<i32>,
}
impl Packet for PacketSetCompression {
    fn read<R: Read>(&mut self, reader: &mut R) {
        self.threshhold = Some(read_varint(reader).unwrap());
    }

    fn write<W: Write>(&self, writer: &mut W) {
        if let Some(threshhold) = self.threshhold {
            write_varint(&threshhold, writer);
        }
    }

    fn get_id(&self) -> i32 {
        3
    }
}

impl Default for PacketSetCompression {
    fn default() -> Self {
        Self { threshhold: None }
    }
}
