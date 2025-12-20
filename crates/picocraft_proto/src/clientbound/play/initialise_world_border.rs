use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x2a)]
pub struct InitialiseWorldBorderPacket {
    pub x: Double,
    pub z: Double,
    pub old_diameter: Double,
    pub new_diameter: Double,
    pub speed: VarLong,
    pub portal_teleport_boundary: VarInt,
    pub warning_blocks: VarInt,
    pub warning_time: VarInt,
}

impl Default for InitialiseWorldBorderPacket {
    fn default() -> Self {
        Self {
            x: 0_f64,
            z: 0_f64,
            old_diameter: 256_f64,
            new_diameter: 256_f64,
            speed: VarLong(0),
            portal_teleport_boundary: VarInt(29999984),
            warning_blocks: VarInt(0),
            warning_time: VarInt(0),
        }
    }
}
