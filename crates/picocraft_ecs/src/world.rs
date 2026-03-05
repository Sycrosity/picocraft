use core::num::NonZeroU16;

use crate::prelude::*;

pub const MAX_PLAYERS: usize = 8;

pub struct World {
    pub entities: Vec<Entity, 1024>,
    pub players: Vec<Entity, MAX_PLAYERS>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            players: Vec::new(),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Entity {
    pub id: u16,
}

pub struct SparseSet<const N: usize> {
    pub dense: Vec<Entity, N>,
    pub sparse: Vec<Option<NonZeroU16>, N>,
}
