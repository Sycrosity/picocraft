use crate::prelude::*;

#[derive(Debug, Default, Packet, bon::Builder)]
#[packet(id = 0x46, state = State::Play)]
pub struct SynchronisePlayerPositionPacket {
    #[builder(default)]
    id: VarInt,
    x: Double,
    y: Double,
    z: Double,
    #[builder(default)]
    velocity_x: Double,
    #[builder(default)]
    velocity_y: Double,
    #[builder(default)]
    velocity_z: Double,
    #[builder(default)]
    yaw: Float,
    #[builder(default)]
    pitch: Float,
    #[builder(default)]
    flags: TeleportFlags,
}

/// Bitfield representing which fields are relative in a teleport packet,
/// however we don't do relative teleportation yet, and so it is always 0.
#[derive(Debug, Clone, Default, Copy, Encode, Decode)]
pub struct TeleportFlags(Int);
