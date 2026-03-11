use picocraft_ecs::prelude::*;

use crate::prelude::*;

pub fn print_players_every_second(world: &mut World) {
    if world.tick_count() % 20 == 0 {
        info!("Player count: {}", world.players.count());

        world.players.username.iter().for_each(|(id, username)| {
            info!("Player: {:?} with ID {}", username.0, id);
        });
    }
}
