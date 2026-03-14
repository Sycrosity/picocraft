use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x01)]
pub struct SpawnEntityPacket {
    pub entity_id: VarInt,
    pub entity_uuid: UUID,
    /// ID in the `minecraft:entity_type` registry
    pub entity_type: EntityType,
    pub x: Double,
    pub y: Double,
    pub z: Double,
    pub velocity: LpVec3,
    pub pitch: Angle,
    pub yaw: Angle,
    pub head_yaw: Angle,
    pub data: VarInt,
}

impl SpawnEntityPacket {
    pub fn player(id: VarInt, uuid: UUID, x: Double, y: Double, z: Double) -> Self {
        Self {
            entity_id: id,
            entity_uuid: uuid,
            entity_type: EntityType::Player,
            x,
            y,
            z,
            velocity: LpVec3::default(),
            pitch: Angle(0),
            yaw: Angle(0),
            head_yaw: Angle(0),
            data: VarInt(0),
        }
    }
}

/// These change every version, and realistically should be generated from the
/// registry, or have a tag for which version they are from when trying to
/// update the values.
#[derive(Debug, Encode, Decode)]
#[protocol(value = VarInt)]
pub enum EntityType {
    Arrow = 6,
    Chicken = 25,
    Cow = 29,
    Creeper = 31,
    Egg = 38,
    Enderman = 40,
    /// This has to be spawned with the `SpawnExperienceOrb` packet.
    ExperienceOrb = 48,

    FallingBlock = 50,
    Horse = 65,
    OakBoat = 87,
    Pig = 97,
    Player = 151,
    Salmon = 107,
    Skeleton = 112,
    Wolf = 145,
    Zombie = 147,
}
