use std::io::{Read, Write};

use super::super::*;

pub struct PacketLoginStart {
    pub username: Option<String>,
}
impl Packet for PacketLoginStart {
    fn read<R: Read>(&mut self, reader: &mut R) {
        if let Ok(username) = read_String(reader) {
            self.username = Some(username);
        }
    }

    fn write<W: Write>(&self, _writer: &mut W) {
        todo!();
    }

    fn get_id(&self) -> i32 {
        0
    }
}

impl Default for PacketLoginStart {
    fn default() -> Self {
        Self { username: None }
    }
}
