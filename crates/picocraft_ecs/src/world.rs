use crate::pools::*;

pub struct World<
    const MAX_PLAYERS: usize = 8,
    const MAX_SAVED_PLAYERS: usize = 16,
    const MAX_MOBS: usize = 128,
    const MAX_PROJECTILES: usize = 128,
> {
    pub players: PlayerPool<MAX_PLAYERS>,
    // pub mobs: MobPool<MAX_MOBS>,
    pub player_save_data: [Option<PlayerSaveData>; MAX_SAVED_PLAYERS],
    tick_count: u64,
}

impl<const MAX_PLAYERS: usize, const MAX_SAVED_PLAYERS: usize>
    World<MAX_PLAYERS, MAX_SAVED_PLAYERS>
{
    pub fn new() -> Self {
        Self {
            players: PlayerPool::new(),
            player_save_data: [const { None }; MAX_SAVED_PLAYERS],
            tick_count: 0,
        }
    }

    /// Increments the world's tick count. This should be called once per game
    /// tick, before any systems are run.
    pub fn increment_tick(&mut self) {
        self.tick_count += 1;
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
