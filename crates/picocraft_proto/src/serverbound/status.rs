use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00, state = State::Status)]
pub struct StatusRequestPacket;

#[derive(Debug, Packet)]
#[packet(id = 0x01, state = State::Status)]
pub struct PingRequestPacket {
    pub timestamp: Long,
}
