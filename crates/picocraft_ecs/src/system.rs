use crate::world::EcsWorld;

/// A system that operates on the mutable ECS world.
///
/// Systems encapsulate game logic — for example, a movement system reads
/// `Position` and `Velocity` components and updates positions accordingly.
///
/// The const generics `MAX_ENTITIES` and `UNIVERSE` must match the world
/// they operate on.
pub trait System<const MAX_ENTITIES: usize, const UNIVERSE: usize> {
    /// Executes this system against the given world state.
    fn run(&self, world: &mut EcsWorld<MAX_ENTITIES, UNIVERSE>);
}
