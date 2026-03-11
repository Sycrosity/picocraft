// #[cfg(feature = "debug")]
pub mod debug;

use picocraft_ecs::components::*;
use picocraft_ecs::entity::EntityId;
use picocraft_ecs::events::WorldEvent;
use picocraft_ecs::pools::PlayerBundle;
use picocraft_ecs::storage::{ComponentStore, GetComponent};
use picocraft_ecs::traits::Pool;
use picocraft_ecs::{ComponentStorageError, World};

use crate::channels::EVENTS;
use crate::prelude::*;

pub fn system_player_moved(
    _world: &mut World,
    _player_id: EntityId,
    _position: Position,
    _rotation: Rotation,
    _on_ground: bool,
) {
    todo!()
}

pub fn system_player_joined(world: &mut World, username: String<16>, uuid: UUID) {
    let bundle = PlayerBundle {
        uuid: Uuid(uuid),
        username: Username(username.clone()),
        health: Health(20f32),
        position: Position::new(0f32, 96f32, 0f32),
        rotation: Rotation::default(),
        velocity: Velocity::default(),
    };

    let mut player = match world.players.spawn(bundle) {
        Ok(entity_ref) => {
            EVENTS
                .immediate_publisher()
                .publish_immediate(WorldEvent::PlayerJoined {
                    player_id: entity_ref.entity_id,
                    username,
                    uuid,
                });

            entity_ref
        }
        // This shouldn't be a possible outcome? I don't think?
        Err(ComponentStorageError::PoolFull) => {
            panic!("Player pool is full");
        }
        Err(e) => {
            error!("Failed to spawn player entity: {e}");
            return;
        }
    };

    player.insert(OnGround).unwrap();
}

//doesn't work correctly yet for some reason
pub fn system_player_left(world: &mut World, player_id: EntityId) {
    let index = player_id.index();

    if !world.players.canonical().contains(index) {
        error!(
            "\"{:?}\" does not correspond to an active player.",
            player_id
        );
        return;
    }

    //TODO don't unwrap
    world.players.despawn(player_id).unwrap();

    EVENTS
        .immediate_publisher()
        .publish_immediate(WorldEvent::PlayerLeft { player_id });
}
