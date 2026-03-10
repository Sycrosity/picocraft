use crate::components::*;
use crate::entity::EntityId;
use crate::prelude::*;

pub enum WorldCommand {
    PlayerJoined {
        client_id: u8,
        username: String<16>,
        uuid: Uuid,
    },
    PlayerLeft {
        client_id: u8,
    },
    PlayerMoved {
        player: EntityId,
        position: Position,
        rotation: Rotation,
        on_ground: bool,
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
        player: EntityId,
        message: String<256>,
    },
    ChatCommand {
        player: EntityId,
        command: String<256>,
    },
}
