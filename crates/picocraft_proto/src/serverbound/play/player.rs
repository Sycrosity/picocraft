use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x1d)]
pub struct SetPlayerPositionPacket {
    pub x: Double,
    pub feet_y: Double,
    pub z: Double,
    pub flags: PlayerMovementFlags,
}

#[derive(Debug, Packet)]
#[packet(id = 0x1f)]
pub struct SetPlayerRotationPacket {
    // Absolute rotation on the X Axis, in degrees.
    pub yaw: Float,
    // Absolute rotation on the Y Axis, in degrees.
    pub pitch: Float,
    pub flags: PlayerMovementFlags,
}

#[derive(Debug, Packet)]
#[packet(id = 0x1e)]
pub struct SetPlayerPositionAndRotationPacket {
    pub x: Double,
    pub feet_y: Double,
    pub z: Double,
    /// Absolute rotation on the X Axis, in degrees.
    pub yaw: Float,
    /// Absolute rotation on the Y Axis, in degrees.
    pub pitch: Float,
    pub flags: PlayerMovementFlags,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct PlayerMovementFlags(u8);

bitflags::bitflags! {
    impl PlayerMovementFlags: u8 {
        const TOUCHING_GROUND = 0x01;
        const TOUCHING_WALL = 0x02;
    }
}

//TODO maybe should be in picocraft_core?
#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
pub struct ProtocolPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
