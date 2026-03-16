use picocraft_derive::EntityPool;

use crate::components::*;
use crate::entity::EntityKind;
use crate::storage::{MarkerSet, SparseSet};

#[derive(Default, EntityPool)]
#[pool(kind = EntityKind::Player)]
pub struct PlayerPool<const N: usize = 8> {
    #[canonical]
    pub uuid: SparseSet<Uuid, N>,
    #[required]
    #[persistent]
    pub username: SparseSet<Username, N>,
    #[required]
    #[persistent]
    pub health: SparseSet<Health, N>,
    #[required]
    #[persistent]
    pub position: SparseSet<Position, N>,
    pub velocity: SparseSet<Velocity, N>,
    #[required]
    #[persistent]
    pub rotation: SparseSet<Rotation, N>,
    #[persistent]
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
