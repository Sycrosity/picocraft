use picocraft_ecs::prelude::*;
use picocraft_proto::serverbound::{
    PlayerMovementFlags, SetPlayerPositionAndRotationPacket, SetPlayerPositionPacket,
    SetPlayerRotationPacket,
};

use crate::channels::COMMANDS;
use crate::prelude::*;

impl HandlePacket for SetPlayerPositionPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        COMMANDS
            .send(WorldCommand::PlayerMoved {
                player_id: client
                    .entity_id
                    .expect("should have an entityId from the ECS"),
                position: Position::from_protocol(self.x, self.feet_y, self.z),
                on_ground: self.flags.contains(PlayerMovementFlags::TOUCHING_GROUND),
                against_wall: self.flags.contains(PlayerMovementFlags::TOUCHING_WALL),
            })
            .await;

        Ok(())
    }
}

impl HandlePacket for SetPlayerRotationPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        COMMANDS
            .send(WorldCommand::PlayerRotated {
                player_id: client
                    .entity_id
                    .expect("should have an entityId from the ECS"),
                rotation: Rotation::new(self.yaw, self.pitch),
                on_ground: self.flags.contains(PlayerMovementFlags::TOUCHING_GROUND),
                against_wall: self.flags.contains(PlayerMovementFlags::TOUCHING_WALL),
            })
            .await;

        Ok(())
    }
}

impl HandlePacket for SetPlayerPositionAndRotationPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        COMMANDS
            .send(WorldCommand::PlayerMovedAndRotated {
                player_id: client
                    .entity_id
                    .expect("should have an entityId from the ECS"),
                position: Position::from_protocol(self.x, self.feet_y, self.z),
                rotation: Rotation::new(self.yaw, self.pitch),
                on_ground: self.flags.contains(PlayerMovementFlags::TOUCHING_GROUND),
                against_wall: self.flags.contains(PlayerMovementFlags::TOUCHING_WALL),
            })
            .await;

        Ok(())
    }
}
