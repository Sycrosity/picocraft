use crate::components::*;
use crate::entity::EntityId;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum WorldEvent {
    PlayerJoined {
        entity_id: EntityId,
        username: String<16>,
        uuid: Uuid,
    },
    PlayerRejected {
        client_id: u8,
        reason: String<64>,
    },
    PlayerLeft {
        entity_id: EntityId,
    },
    BlockBroken {
        player: EntityId,
        pos: BlockPosition,
    },
    BlockPlaced {
        player: EntityId,
        pos: BlockPosition,
    },
}

pub enum Recipient {
    Player(EntityId),    // health, inventory, ack
    AllExcept(EntityId), // movement
    All,                 // joins, deaths, chat, block mutations
}
