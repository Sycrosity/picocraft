use crate::pools::*;

#[derive(Default)]
pub struct World<
    const MAX_PLAYERS: usize = 8,
    const MAX_MOBS: usize = 128,
    const MAX_PROJECTILES: usize = 128,
> {
    pub players: PlayerPool<MAX_PLAYERS>,
    // pub mobs: MobPool<MAX_MOBS>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
}
