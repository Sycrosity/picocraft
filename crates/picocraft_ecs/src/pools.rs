use picocraft_derive::EntityPool;

use crate::components::*;
use crate::entity::EntityKind;
use crate::storage::{MarkerSet, SparseSet};

#[derive(Default, EntityPool)]
#[pool(kind = EntityKind::Player)]
pub struct PlayerPool<const N: usize> {
    #[entity(canonical)]
    pub uuid: SparseSet<Uuid, N>,
    #[entity(persistent)]
    pub username: SparseSet<Username, N>,
    #[entity(persistent)]
    pub health: SparseSet<Health, N>,
    #[entity(persistent)]
    pub position: SparseSet<Position, N>,
    pub velocity: SparseSet<Velocity, N>,
    #[entity(persistent)]
    pub rotation: SparseSet<Rotation, N>,
    #[entity(persistent)]
    pub dimension: SparseSet<Dimension, N>,
    #[entity(optional)]
    pub on_ground: MarkerSet<OnGround, N>,
    #[entity(optional)]
    pub fall_distance: SparseSet<FallDistance, N>,
}

impl<const N: usize> PlayerPool<N> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
