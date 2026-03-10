use super::pools::*;
use crate::prelude::*;

pub const MAX_PLAYERS: usize = 8;

#[derive(Default)]
pub struct World {
    pub players: PlayerPool<8>,
    // pub mobs: MobPool<128>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
}
