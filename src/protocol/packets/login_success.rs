use std::io::{Read, Write};

use uuid::Uuid;

use super::super::*;

pub struct PacketLoginSuccess {
    pub uuid: Option<Uuid>,
    pub username: Option<String>,
}
impl Packet for PacketLoginSuccess {
    fn write<W: Write>(&self, writer: &mut W) {
        write_u128(&self.uuid.expect("Expected UUID").as_u128(), writer);
        write_String(&self.username.clone().expect("Expected Username"), writer);
    }

    fn get_id(&self) -> i32 {
        2
    }

    fn read<R: Read>(&mut self, reader: &mut R) {
        self.uuid = Some(Uuid::from_u128(read_u128(reader).unwrap()));
        self.username = Some(read_String(reader).unwrap())
    }
}

impl Default for PacketLoginSuccess {
    fn default() -> Self {
        Self {
            uuid: None,
            username: None,
        }
    }
}
