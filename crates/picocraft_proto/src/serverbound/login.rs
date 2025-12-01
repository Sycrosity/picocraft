use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct LoginStartPacket {
    pub username: String<16>,
    pub uuid: UUID,
}

#[derive(Debug, Packet)]
#[packet(id = 0x03)]
pub struct LoginAcknowledgedPacket;
