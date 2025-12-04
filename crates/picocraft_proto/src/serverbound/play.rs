use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct ConfirmTeleportationPacket(pub VarInt);
