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

pub enum MovementUpdate {
    Nearby(DeltaPosition),
    Teleport(Position),
}

impl MovementUpdate {
    pub fn from_positions(old: Position, new: Position) -> Self {
        let dx = ((new.x as f64 - old.x as f64) * 4096.0) as i64;
        let dy = ((new.y as f64 - old.y as f64) * 4096.0) as i64;
        let dz = ((new.z as f64 - old.z as f64) * 4096.0) as i64;

        if let (Ok(dx), Ok(dy), Ok(dz)) = (i16::try_from(dx), i16::try_from(dy), i16::try_from(dz))
        {
            Self::Nearby(DeltaPosition { dx, dy, dz })
        } else {
            Self::Teleport(new)
        }
    }
}

pub fn system_player_moved(
    world: &mut World,
    player_id: EntityId,
    position: Option<Position>,
    rotation: Option<Rotation>,
    on_ground: bool,
    against_wall: bool,
) {
    let movement_update = if let Some(new_position) = position
        && let Some(current_position) = world.players.position.get_mut(player_id.index())
    {
        let movement_update = MovementUpdate::from_positions(*current_position, new_position);
        *current_position = new_position;

        Some(movement_update)
    } else {
        None
    };

    let current_rotation = world
        .players
        .rotation
        .get_mut(player_id.index())
        .expect("rotation should be a required field");

    let old_rotation = *current_rotation;

    if let Some(new_rotation) = rotation {
        *current_rotation = new_rotation;
    };

    let event = match (movement_update, rotation) {
        (Some(MovementUpdate::Nearby(delta_position)), opt_rotation) => match opt_rotation {
            Some(rotation) => WorldEvent::PlayerMovedAndRotated {
                player_id,
                delta_position,
                rotation,
                on_ground,
                against_wall,
            },
            None => WorldEvent::PlayerMoved {
                player_id,
                delta_position,
                on_ground,
                against_wall,
            },
        },
        (Some(MovementUpdate::Teleport(position)), opt_rotation) => WorldEvent::PlayerTeleported {
            player_id,
            position,
            rotation: opt_rotation.unwrap_or(old_rotation),
            on_ground,
        },
        (None, Some(rotation)) => WorldEvent::PlayerRotated {
            player_id,
            rotation,
            on_ground,
            against_wall,
        },
        (None, None) => {
            unreachable!("system_player_moved called without position or rotation update");
        }
    };

    EVENTS.immediate_publisher().publish_immediate(event);
}

pub fn system_player_joined(world: &mut World, username: String<16>, uuid: UUID) {
    let existing_players: Vec<_, MAX_PLAYERS> = world
        .players
        .uuid
        .iter()
        .map(|(index, uuid)| {
            // these are all required fields, so we know they exist for every player
            let username = world
                .players
                .username
                .get(index)
                .expect("username should be a required field");
            let position = world
                .players
                .position
                .get(index)
                .expect("position should be a required field");
            let rotation = world
                .players
                .rotation
                .get(index)
                .expect("rotation should be a required field");
            (
                EntityId::player(index),
                username.0.clone(),
                uuid.0,
                *position,
                *rotation,
            )
        })
        .collect();

    let save = world
        .player_save_data
        .iter()
        .find_map(|slot| slot.as_ref().filter(|s| s.uuid == Uuid(uuid)).cloned());

    let (position, rotation) = match &save {
        Some(save) => (save.position, save.rotation),
        None => (Position::new(0.0, 96.0, 0.0), Rotation::default()),
    };

    let entity_ref = match save {
        Some(save) => world.players.restore(save),
        None => world.players.spawn(PlayerBundle {
            uuid: Uuid(uuid),
            username: Username(username.clone()),
            health: Health(20.0),
            //TODO new player spawn pos should come from terrain and random number gen
            position,
            rotation,
        }),
    };

    let mut player = match entity_ref {
        Ok(entity_ref) => {
            EVENTS
                .immediate_publisher()
                .publish_immediate(WorldEvent::PlayerJoined {
                    player_id: entity_ref.entity_id,
                    username,
                    uuid,
                    position,
                    rotation,
                });

            entity_ref
        }
        // This shouldn't be a possible outcome? I don't think?
        Err(ComponentStorageError::PoolFull) => {
            unreachable!("Player pool is full");
        }
        Err(e) => {
            error!("Failed to spawn player entity: {e}");
            return;
        }
    };

    player
        .insert(Dimension::Overworld)
        .expect("EntityId should be valid");

    for (player_id, username, uuid, pos, rot) in existing_players {
        EVENTS
            .immediate_publisher()
            .publish_immediate(WorldEvent::ExistingPlayer {
                recipient: player.entity_id,
                player_id,
                username,
                uuid,
                position: pos,
                rotation: rot,
            });
    }

    EVENTS
        .immediate_publisher()
        .publish_immediate(WorldEvent::WorldReady {
            recipient: player.entity_id,
        });
}

pub fn system_player_left(world: &mut World, player_id: EntityId) {
    let index = player_id.index();

    if !world.players.canonical().contains(index) {
        error!(
            "\"{:?}\" does not correspond to an active player.",
            player_id
        );
        return;
    }

    let uuid = world
        .players
        .uuid
        .get(player_id.index())
        .expect("UUID should be the canonical component")
        .0;

    if let Some(save) = world.players.snapshot(player_id) {
        // find the slot by UUID and store it
        if let Some(slot) = world
            .player_save_data
            .iter_mut()
            .find(|s| s.as_ref().map(|s| s.uuid == save.uuid).unwrap_or(false))
        {
            *slot = Some(save);
        } else if let Some(slot) = world.player_save_data.iter_mut().find(|s| s.is_none()) {
            *slot = Some(save);
        }
    }

    if let Err(e) = world.players.despawn(player_id) {
        error!("Failed to despawn player entity: {e}");
        return;
    }

    EVENTS
        .immediate_publisher()
        .publish_immediate(WorldEvent::PlayerLeft { player_id, uuid });
}
