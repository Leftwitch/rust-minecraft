use std::{io, net::TcpStream, sync::Arc, thread};

use uuid::Uuid;

use crate::protocol::{
    Packet, PacketHandshake, PacketLoginStart, PacketLoginSuccess, PacketPingPong,
    PacketServerList, PacketSetCompression,
};

use super::minecraft_connection::*;

enum ConnectionState {
    HANDSHAKING,
    STATUS,
    PING,
    LOGIN,
    PROXY,
    FINISHED,
}

enum ProxyState {
    CONNECTING,
    PROXYING,
}

struct Handshake {
    protocol_version: i32,
    server_address: String,
    port: u16,
}

impl Handshake {
    fn to_packet(&self) -> PacketHandshake {
        PacketHandshake {
            protocol_version: Some(self.protocol_version),
            server_address: Some(self.server_address.clone()),
            port: Some(self.port),
            next_state: Some(2),
        }
    }
}
pub struct ProxiedConnection {
    user_connection: MinecraftConnection,
    server_connection: Option<MinecraftConnection>,
    connection_state: ConnectionState,
    proxy_state: ProxyState,
    handshake: Option<Handshake>,
    username: Option<String>,
}

impl ProxiedConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            user_connection: MinecraftConnection::new(stream),
            server_connection: None,
            connection_state: ConnectionState::HANDSHAKING,
            proxy_state: ProxyState::CONNECTING,
            handshake: None,
            username: None,
        }
    }

    pub fn connect_to_server(&mut self, server: MinecraftConnection) {
        self.proxy_state = ProxyState::CONNECTING;
        self.server_connection = Some(server);
        let handshake = self
            .handshake
            .as_ref()
            .expect("No initial Handshake Packet sent?");

        let username = self
            .username
            .as_ref()
            .expect("Proxied connection without username ?");

        let login_request = PacketLoginStart {
            username: Some(username.clone()),
        };
        let mut server_connection = self.server_connection.as_mut().unwrap();
        server_connection.send_packet(&handshake.to_packet());
        server_connection.send_packet(&login_request);

        self.proxy_state = ProxyState::CONNECTING;

        //TODO HANDLE COMPRESSION
        println!("Waiting for Login Success by Server...");

        let packet_id = server_connection.recv_packet();
        println!("Recieved Packet: {}", packet_id);
        if packet_id == 2 {
            println!("Login for user {} success! ", username);

            let mut login_success = PacketLoginSuccess::default();
            login_success.read(&mut server_connection);

            if let Some(username) = login_success.username {
                println!("Login for user {} success! ", username);
                self.proxy_state = ProxyState::PROXYING;
            }
        }
    }

    pub fn handle_connection(&mut self) {
        loop {
            match self.connection_state {
                ConnectionState::HANDSHAKING => {
                    let packet_id = self.user_connection.recv_packet();
                    println!("PACKET ID RECEIVED: {}", packet_id);
                    if packet_id == 0 {
                        println!("READ HANDSHAKE");

                        self.handle_handshake();
                    }
                }
                ConnectionState::STATUS => {
                    let packet_id = self.user_connection.recv_packet();
                    if packet_id == 0 {
                        println!("READ STATUS");
                        let status = PacketServerList {};
                        self.user_connection.send_packet(&status);
                        self.connection_state = ConnectionState::PING;
                        println!("NEXT STATE PING");
                    }
                }
                ConnectionState::PING => {
                    let packet_id = self.user_connection.recv_packet();
                    if packet_id == 1 {
                        println!("READ PING");
                        let mut ping = PacketPingPong::default();
                        ping.read(&mut self.user_connection);

                        self.user_connection.send_packet(&ping);
                        self.connection_state = ConnectionState::FINISHED;
                        println!("NEXT STATE FINISHED");
                    }
                }
                ConnectionState::LOGIN => {
                    let packet_id = self.user_connection.recv_packet();
                    if packet_id == 0 {
                        println!("READ LOGIN");
                        let mut login_start = PacketLoginStart::default();
                        login_start.read(&mut self.user_connection);
                        self.username = login_start.username;

                        let login_success = PacketLoginSuccess {
                            username: self.username.clone(),
                            uuid: Some(Uuid::new_v4()),
                        };

                        self.user_connection.send_packet(&login_success);
                        self.connection_state = ConnectionState::PROXY;
                        println!("NEXT STATE PROXY");

                        let server_connection = TcpStream::connect("172.18.48.1:25564").unwrap();
                        let minecraft_connection = MinecraftConnection::new(server_connection);
                        self.connect_to_server(minecraft_connection);
                    }
                }
                ConnectionState::PROXY => {
                    if let ProxyState::PROXYING = self.proxy_state {
                        //Do Proxy stuff
                        self.proxy_traffic();
                    }
                }
                ConnectionState::FINISHED => {
                    println!("Connection Finished");
                    break;
                }
            }
        }
    }
}
impl ProxiedConnection {
    fn handle_handshake(&mut self) {
        let mut handshake = PacketHandshake::default();
        handshake.read(&mut self.user_connection);
        println!("HANDSHAKE: {:?}", handshake);

        self.handshake = Some(Handshake {
            protocol_version: handshake.protocol_version.unwrap(),
            port: handshake.port.unwrap(),
            server_address: handshake.server_address.unwrap(),
        });
        let next_state = handshake.next_state.unwrap();

        if next_state == 1 {
            self.connection_state = ConnectionState::STATUS;
            println!("NEXT STATE STATUS");
        } else if next_state == 2 {
            self.connection_state = ConnectionState::LOGIN;
            println!("NEXT STATE LOGIN");
        }
    }

    fn proxy_traffic(&mut self) {
        let server_connection = self.server_connection.take().unwrap();
        let lhs_arc = Arc::new(server_connection.get_stream());
        let rhs_arc = Arc::new(self.user_connection.get_stream());

        let (mut lhs_tx, mut lhs_rx) = (lhs_arc.try_clone().unwrap(), lhs_arc.try_clone().unwrap());
        let (mut rhs_tx, mut rhs_rx) = (rhs_arc.try_clone().unwrap(), rhs_arc.try_clone().unwrap());

        let connections = vec![
            thread::spawn(move || io::copy(&mut lhs_tx, &mut rhs_rx).unwrap()),
            thread::spawn(move || io::copy(&mut rhs_tx, &mut lhs_rx).unwrap()),
        ];

        for t in connections {
            t.join().unwrap();
        }
    }
}
