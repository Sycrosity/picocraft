use picocraft_ecs::commands::WorldCommand;
use picocraft_ecs::world::World;
use picocraft_terrain::Terrain;

use crate::channels::COMMANDS;
use crate::systems::*;

pub fn tick(world: &mut World, _terrain: &Terrain) {
    world.increment_tick();

    // drain commands first to mutate world state before any systems run
    while let Ok(cmd) = COMMANDS.try_receive() {
        handle_command(world, cmd);
    }

    debug::print_players_every_second(world);

    // // systems
    // physics::apply_velocity_players(&mut world.players);
    // physics::apply_velocity_mobs(&mut world.mobs);
    // physics::update_on_ground_players(&mut world.players, terrain);
    // physics::update_on_ground_mobs(&mut world.mobs, terrain);
    // physics::accumulate_fall_distance(&mut world.mobs);
    // physics::fall_damage(&mut world.mobs);
    // physics::fall_damage(&mut world.players);

    // ai::mob_ai(world);

    // combat::attack_cooldown(&mut world.mobs);
    // combat::apply_damage(&mut world.mobs);
    // combat::apply_damage(&mut world.players);
}

#[allow(unreachable_patterns)]
fn handle_command(world: &mut World, cmd: WorldCommand) {
    match cmd {
        WorldCommand::PlayerMoved {
            player_id,
            position,
            rotation,
            on_ground,
        } => {
            system_player_moved(world, player_id, position, rotation, on_ground);
        }
        WorldCommand::PlayerJoined { username, uuid } => {
            system_player_joined(world, username, uuid);
        }
        WorldCommand::PlayerLeft { player_id } => {
            system_player_left(world, player_id);
        }
        _ => {}
    }
}
