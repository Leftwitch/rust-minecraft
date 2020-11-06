use std::{io::Read, io::Write};

use super::super::*;
#[derive(Debug)]
pub struct PacketHandshake {
    pub protocol_version: Option<i32>,
    pub server_address: Option<String>,
    pub port: Option<u16>,
    pub next_state: Option<i32>,
}

impl Packet for PacketHandshake {
    fn read<R: Read>(&mut self, reader: &mut R) {
        self.protocol_version = Some(read_varint(reader).unwrap());
        self.server_address = Some(read_String(reader).unwrap());
        self.port = Some(read_u16(reader).unwrap());
        self.next_state = Some(read_varint(reader).unwrap());
    }

    fn write<W: Write>(&self, writer: &mut W) {
        write_varint(
            self.protocol_version
                .as_ref()
                .expect("Protocol version not sot?"),
            writer,
        );

        write_String(
            self.server_address.as_ref().expect("ServerAdress not set?"),
            writer,
        );
        write_u16(self.port.as_ref().expect("Port not set?"), writer);

        write_varint(
            self.next_state
                .as_ref()
                .expect("NextState version not sot?"),
            writer,
        );
    }

    fn get_id(&self) -> i32 {
        0
    }
}

impl Default for PacketHandshake {
    fn default() -> Self {
        Self {
            protocol_version: None,
            server_address: None,
            port: None,
            next_state: None,
        }
    }
}
