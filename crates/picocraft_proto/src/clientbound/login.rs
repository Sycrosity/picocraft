use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct LoginDisconnectPacket {
    pub reason: String<256>,
    // reason: TextComponent
}

#[derive(Debug, Packet)]
#[packet(id = 0x02)]
pub struct LoginSuccess(pub GameProfile);
