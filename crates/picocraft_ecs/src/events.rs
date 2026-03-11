use crate::MAX_PLAYERS;
use crate::entity::EntityId;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum WorldEvent {
    PlayerJoined {
        player_id: EntityId,
        username: String<16>,
        uuid: UUID,
    },
    PlayerLeft {
        player_id: EntityId,
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
        message: String<256>,
    },
}

pub enum Recipient {
    Player(EntityId),    // health, inventory, ack
    AllExcept(EntityId), // movement
    All,                 // joins, deaths, chat, block mutations
}

impl WorldEvent {
    pub fn recipient(&self) -> Recipient {
        match self {
            Self::PlayerJoined { player_id, .. } => Recipient::All,
            Self::PlayerLeft { player_id, .. } => Recipient::All,
            // Self::PlayerMoved  { player_id, .. }  => Recipient::AllExcept(*player_id),
            // Self::BlockBroken  { player_id, .. }  => Recipient::AllExcept(*player_id),
            // Self::BlockPlaced  { player_id, .. }  => Recipient::AllExcept(*player_id),
            // Self::PlayerDamaged { player_id, .. } => Recipient::All,
            // Self::PlayerDied   { player_id, .. }  => Recipient::All,
            Self::ChatMessage { player_id, .. } => Recipient::AllExcept(*player_id),
        }
    }
}
