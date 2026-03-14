use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x33)]
pub struct UpdateEntityPosPacket {
    pub entity_id: VarInt,
    pub delta_x: Short,
    pub delta_y: Short,
    pub delta_z: Short,
    pub on_ground: Boolean,
}

pub struct UpdateEntityPositionandRotationPacket {
    pub entity_id: VarInt,
    pub delta_x: Short,
    pub delta_y: Short,
    pub delta_z: Short,
    /// New angle value, not a delta
    pub yaw: Byte,
    /// New angle value not a delta
    pub pitch: Byte,
    pub on_ground: Boolean,
}
