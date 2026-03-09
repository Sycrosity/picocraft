use super::component_store::{MarkerSet, SparseSet};
use super::components::*;
use crate::prelude::*;

#[derive(Default)]
pub struct PlayerPool<const N: usize = 8> {
    pub health: SparseSet<Health, N>,
    pub on_ground: MarkerSet<OnGround, N>,
    pub velocity: SparseSet<Velocity, N>,
    pub position: SparseSet<Position, N>,
    pub rotation: SparseSet<Rotation, N>,
    pub fall_distance: SparseSet<FallDistance, N>,
    pub uuid: SparseSet<Uuid, N>,
}

impl<const N: usize> PlayerPool<N> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
