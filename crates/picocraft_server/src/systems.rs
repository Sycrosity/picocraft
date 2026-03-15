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

    // Either we have a new rotation to set, or we keep the existing rotation (e.g.
    // if this was just a position update). This is because `TeleportEntity` packets
    // still require a rotation value, even if it doesn't change.
    let rotation_to_send = if let Some(new_rotation) = rotation {
        *current_rotation = new_rotation;

        new_rotation
    } else {
        *current_rotation
    };

    let event = match (movement_update, rotation) {
        (Some(MovementUpdate::Nearby(delta_position)), opt_rotation) => match opt_rotation {
            Some(rotation) => WorldEvent::PlayerMovedAndRotated {
                player_id,
                delta_position,
                rotation: rotation_to_send,
                on_ground,
                against_wall,
            },
            None => WorldEvent::PlayerMoved {
                player_id,
                delta_position,
                rotation: rotation_to_send,
                on_ground,
                against_wall,
            },
        },
        (Some(MovementUpdate::Teleport(position)), _) => WorldEvent::PlayerTeleported {
            player_id,
            position,
            rotation: rotation_to_send,
            on_ground,
        },
        (None, Some(rotation)) => WorldEvent::PlayerRotated {
            player_id,
            rotation: rotation_to_send,
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
    let bundle = PlayerBundle {
        uuid: Uuid(uuid),
        username: Username(username.clone()),
        health: Health(20f32),
        position: Position::new(0f32, 96f32, 0f32),
        rotation: Rotation::default(),
        velocity: Velocity::default(),
    };

    let mut player = match world.players.spawn(bundle.clone()) {
        Ok(entity_ref) => {
            EVENTS
                .immediate_publisher()
                .publish_immediate(WorldEvent::PlayerJoined {
                    player_id: entity_ref.entity_id,
                    username,
                    uuid,
                    position: bundle.position,
                    rotation: bundle.rotation,
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

    player
        .insert(Dimension::Overworld)
        .expect("EntityId should be valid");
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

    //TODO don't unwrap
    world.players.despawn(player_id).unwrap();

    EVENTS
        .immediate_publisher()
        .publish_immediate(WorldEvent::PlayerLeft { player_id, uuid });
}
