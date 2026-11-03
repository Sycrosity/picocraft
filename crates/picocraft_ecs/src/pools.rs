use picocraft_derive::EntityPool;

use crate::components::*;
use crate::entity::EntityKind;
use crate::storage::{MarkerSet, SparseSet};

#[derive(Default, EntityPool)]
#[pool(kind = EntityKind::Player)]
pub struct PlayerPool<const N: usize> {
    #[entity(canonical)]
    pub uuid: SparseSet<Uuid, N>,
    #[entity(required, persistent)]
    pub username: SparseSet<Username, N>,
    #[entity(required,persistent)]
    pub health: SparseSet<Health, N>,
    #[entity(required, persistent)]
    pub position: SparseSet<Position, N>,
    pub velocity: SparseSet<Velocity, N>,
    #[entity(required, persistent)]
    pub rotation: SparseSet<Rotation, N>,
    #[entity(required, persistent)]
    pub dimension: SparseSet<Dimension, N>,
    pub on_ground: MarkerSet<OnGround, N>,
    pub fall_distance: SparseSet<FallDistance, N>,
}

impl<const N: usize> PlayerPool<N> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
