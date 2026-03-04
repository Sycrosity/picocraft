use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00)]
pub struct ConfirmTeleportationPacket(pub VarInt);

#[derive(Debug, Packet)]
#[packet(id = 0x0c)]
pub struct ClientTickEndPacket;

#[derive(Debug, Packet)]
#[packet(id = 0x1d)]
pub struct SetPlayerPositionPacket {

    x: Double,
    feet_y: Double,
    z: Double,
    // use bitflags here
    /// 0x00 - nothing
    /// 0x01 - touching the ground
    /// 0x02 - touching a wall
    flags: Byte,

}