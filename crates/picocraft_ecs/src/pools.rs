use picocraft_derive::EntityPool;

use super::components::*;
use super::storage::{MarkerSet, SparseSet};

#[derive(Default, EntityPool)]
#[pool(kind = EntityKind::Player)]
pub struct PlayerPool<const N: usize = 8> {
    #[canonical]
    pub uuid: SparseSet<Uuid, N>,
    #[required]
    pub health: SparseSet<Health, N>,
    #[required]
    pub position: SparseSet<Position, N>,
    #[required]
    pub velocity: SparseSet<Velocity, N>,
    #[required]
    pub rotation: SparseSet<Rotation, N>,
    pub on_ground: MarkerSet<OnGround, N>,
    pub fall_distance: SparseSet<FallDistance, N>,
}

impl<const N: usize> PlayerPool<N> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
