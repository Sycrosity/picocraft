use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct LoginDisconnectPacket {
    // reason: Chat,
}
