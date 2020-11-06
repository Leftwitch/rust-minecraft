use std::io::{Read, Write};

use uuid::Uuid;

use super::super::*;

pub struct PacketLoginSuccess {
    pub uuid: Uuid,
    pub username: String,
}
impl Packet for PacketLoginSuccess {
    fn write<W: Write>(&self, writer: &mut W) {
        write_u128(&self.uuid.as_u128(), writer);
        write_String(&self.username, writer);
    }

    fn get_id(&self) -> i32 {
        2
    }

    fn read<R: Read>(&mut self, _reader: &mut R) {
        todo!()
    }
}
