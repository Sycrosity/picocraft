use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct LoginStartPacket {
    username: String<16>,
    // uuid: UUID,
}
