use std::io::{Read, Write};

use super::super::*;

#[derive(Debug)]
pub struct PacketPingPong {
    pub challenge: Option<i64>,
}
impl Packet for PacketPingPong {
    fn read<R: Read>(&mut self, reader: &mut R) {
        self.challenge = Some(read_i64(reader).unwrap());
    }

    fn write<W: Write>(&self, writer: &mut W) {
        if let Some(challenge) = self.challenge {
            write_i64(&challenge, writer);
        }
    }

    fn get_id(&self) -> i32 {
        1
    }
}

impl Default for PacketPingPong {
    fn default() -> Self {
        Self { challenge: None }
    }
}
