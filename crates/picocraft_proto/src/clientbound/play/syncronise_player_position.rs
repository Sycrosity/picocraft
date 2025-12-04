use crate::prelude::*;

#[derive(Debug, Default, Packet)]
#[packet(id = 0x46, state = State::Play)]
pub struct SynchronisePlayerPositionPacket {
    id: VarInt,
    x: Double,
    y: Double,
    z: Double,
    velocity_x: Double,
    velocity_y: Double,
    velocity_z: Double,
    yaw: Float,
    pitch: Float,
    flags: TeleportFlags,
}

/// Bitfield representing which fields are relative in a teleport packet,
/// however we don't do relative teleportation yet, and so it is always 0.
#[derive(Debug, Clone, Default, Copy, Encode, Decode)]
pub struct TeleportFlags(Int);
