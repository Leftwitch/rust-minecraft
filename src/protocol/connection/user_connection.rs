use uuid::Uuid;

use super::minecraft_connection::MinecraftConnection;
use crate::protocol::{
    get_packet_size, read_varint, write_varint, Packet, PacketHandshake, PacketLoginStart,
    PacketLoginSuccess, PacketPingPong, PacketServerList,
};
use std::net::TcpStream;

pub enum ConnectionState {
    HANDSHAKING,
    STATUS,
    PING,
    LOGIN,
    PLAY,
    FINISHED,
}
pub struct UserConnection {
    stream: TcpStream,
    state: ConnectionState,
    username: Option<String>,
}

impl MinecraftConnection for UserConnection {
    fn send_packet<P: Packet>(&mut self, packet: &P) {
        let size: i32 = get_packet_size(packet) as i32;
        write_varint(&size, &mut self.stream);
        write_varint(&packet.get_id(), &mut self.stream);
        packet.write(&mut self.stream);
    }

    fn recv_packet(&mut self) {
        let packet_length = read_varint(&mut self.stream);
        if let Ok(_packet_length) = packet_length {
            let packet_id = read_varint(&mut self.stream);
            if let Ok(packet_id) = packet_id {
                self.handle_packet(packet_id);
            }
        }
    }
}

impl UserConnection {
    fn handle_packet(&mut self, packet_id: i32) {
        match self.state {
            ConnectionState::HANDSHAKING => {
                if packet_id == 0 {
                    let mut handshake = PacketHandshake::default();
                    handshake.read(&mut self.stream);
                    if let Some(next_state) = handshake.next_state {
                        if next_state == 1 {
                            println!("CURRENT STATE: HANDSHAKING NEW STATE: STATUS");
                            self.state = ConnectionState::STATUS;
                        } else if next_state == 2 {
                            println!("CURRENT STATE: HANDSHAKING NEW STATE: LOGIN");
                            self.state = ConnectionState::LOGIN;
                        }
                    }
                }
            }
            ConnectionState::STATUS => {
                let status = PacketServerList {};
                self.send_packet(&status);
                self.state = ConnectionState::PING;
                println!("CURRENT STATE: STATUS NEW STATE: PING");
            }
            ConnectionState::PING => {
                let mut packet = PacketPingPong::default();
                packet.read(&mut self.stream);
                self.send_packet(&packet);
                self.state = ConnectionState::FINISHED;
                println!("CURRENT STATE: PING NEW STATE: FINISHED");
            }
            ConnectionState::LOGIN => {
                let mut packet = PacketLoginStart::default();
                packet.read(&mut self.stream);

                if let Some(username) = packet.username {
                    println!("Logging in user: {}", username);
                    self.username = Some(username.clone());

                    let packet = PacketLoginSuccess {
                        uuid: Uuid::new_v4(),
                        username: username,
                    };

                    self.send_packet(&packet);
                    println!("CURRENT STATE: LOGIN NEW STATE: PLAY");
                    self.state = ConnectionState::PLAY;
                }
            }
            ConnectionState::PLAY => {
                println!("We are ingame");
            }
            ConnectionState::FINISHED => {
                println!("Done with protocol");
            }
        }
    }
}

impl UserConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            username: None,
            state: ConnectionState::HANDSHAKING,
        }
    }
}
