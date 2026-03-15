mod player;

pub use player::*;

use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct ConfirmTeleportationPacket(pub VarInt);

#[derive(Debug, Packet)]
#[packet(id = 0x0c)]
pub struct ClientTickEndPacket;

#[derive(Debug, Packet)]
#[packet(id = 0x1b)]
pub struct ServerboundKeepAlivePacket {
    pub id: Long,
}
