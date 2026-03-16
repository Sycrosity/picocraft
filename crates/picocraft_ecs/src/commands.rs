use crate::components::*;
use crate::entity::EntityId;
use crate::prelude::*;

/// Commands that can be sent to the world from outside systems, e.g. from
/// network handlers. Conveys intent for the actions.
pub enum WorldCommand {
    /// Doesn't include EntityId because the player hasn't been spawned yet -
    /// the ECS world will assign one when the player is spawned.
    PlayerJoined {
        username: String<16>,
        uuid: UUID,
    },
    PlayerLeft {
        player_id: EntityId,
    },
    PlayerMoved {
        player_id: EntityId,
        position: Position,
        on_ground: bool,
        against_wall: bool,
    },
    PlayerRotated {
        player_id: EntityId,
        rotation: Rotation,
        on_ground: bool,
        against_wall: bool,
    },
    PlayerMovedAndRotated {
        player_id: EntityId,
        position: Position,
        rotation: Rotation,
        on_ground: bool,
        against_wall: bool,
    },
    // BlockBroken {
    //     player: EntityId,
    //     pos: BlockPosition,
    // },
    // BlockPlaced {
    //     player: EntityId,
    //     pos: BlockPosition,
    //     block: Block,
    // },
    ChatMessage {
        player_id: EntityId,
        message: String<128>,
    },
    ChatCommand {
        player_id: EntityId,
        command: String<128>,
    },
}
