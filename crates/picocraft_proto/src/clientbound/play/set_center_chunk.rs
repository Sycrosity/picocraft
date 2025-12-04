use crate::prelude::*;

#[derive(Debug, Default, Packet)]
#[packet(id = 0x5c)]
pub struct SetCenterChunkPacket {
    pub x: VarInt,
    pub z: VarInt,
}

impl SetCenterChunkPacket {
    pub fn new(x: VarInt, z: VarInt) -> Self {
        Self { x, z }
    }
}
