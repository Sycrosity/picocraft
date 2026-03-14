use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x43)]
pub struct PlayerInfoRemovePacket {
    pub uuids: PrefixedArray<UUID, MAX_PLAYERS>,
}
