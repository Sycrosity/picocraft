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

#[derive(Debug, Packet)]
#[packet(id = 0x34)]
pub struct UpdateEntityPositionandRotationPacket {
    pub entity_id: VarInt,
    pub delta_x: Short,
    pub delta_y: Short,
    pub delta_z: Short,
    /// New angle value, not a delta
    pub yaw: Angle,
    /// New angle value not a delta
    pub pitch: Angle,
    pub on_ground: Boolean,
}

#[derive(Debug, Packet)]
// 0x36 is not a mistake.
#[packet(id = 0x36)]
pub struct UpdateEntityRotationPacket {
    pub entity_id: VarInt,
    /// New angle value, not a delta
    pub yaw: Angle,
    /// New angle value not a delta
    pub pitch: Angle,
    pub on_ground: Boolean,
}

#[derive(Debug, Packet)]
#[packet(id = 0x51)]
pub struct SetHeadRotationPacket {
    pub entity_id: VarInt,
    /// New angle value, not a delta
    pub head_yaw: Angle,
}
