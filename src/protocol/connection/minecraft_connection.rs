use super::super::Packet;

pub trait MinecraftConnection {
    fn send_packet<P: Packet>(&mut self, packet: &P);
    fn recv_packet(&mut self);
}
