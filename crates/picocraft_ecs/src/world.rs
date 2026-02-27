use crate::component::*;
use crate::entity::{Entity, EntityAllocator};
use crate::sparse_set::SparseSet;
use crate::system::System;

/// The central ECS world that holds all entities and their components.
///
/// - `MAX_ENTITIES`: maximum number of simultaneously alive entities.
/// - `UNIVERSE`: the key space for sparse sets — must be `>= MAX_ENTITIES`.
///
/// Component sparse sets are stored directly as fields so that the entire
/// world lives on the stack with compile-time-known size.
pub struct EcsWorld<const MAX_ENTITIES: usize, const UNIVERSE: usize> {
    allocator: EntityAllocator<UNIVERSE>,
    pub positions: SparseSet<Position, MAX_ENTITIES, UNIVERSE>,
    pub velocities: SparseSet<Velocity, MAX_ENTITIES, UNIVERSE>,
    pub healths: SparseSet<Health, MAX_ENTITIES, UNIVERSE>,
    pub armours: SparseSet<Armour, MAX_ENTITIES, UNIVERSE>,
    pub entity_kinds: SparseSet<EntityKind, MAX_ENTITIES, UNIVERSE>,
    pub block_interactions: SparseSet<BlockInteraction, MAX_ENTITIES, UNIVERSE>,
    pub pathfinding: SparseSet<PathfindingState, MAX_ENTITIES, UNIVERSE>,
}

impl<const MAX_ENTITIES: usize, const UNIVERSE: usize> EcsWorld<MAX_ENTITIES, UNIVERSE> {
    /// Creates a new empty world.
    pub fn new() -> Self {
        Self {
            allocator: EntityAllocator::new(),
            positions: SparseSet::new(),
            velocities: SparseSet::new(),
            healths: SparseSet::new(),
            armours: SparseSet::new(),
            entity_kinds: SparseSet::new(),
            block_interactions: SparseSet::new(),
            pathfinding: SparseSet::new(),
        }
    }

    /// Spawns a new entity and returns its id, or `None` if at capacity.
    pub fn spawn(&mut self) -> Option<Entity> {
        self.allocator.allocate()
    }

    /// Despawns an entity and removes all of its components.
    ///
    /// Returns `true` if the entity was alive and is now removed.
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.allocator.deallocate(entity) {
            return false;
        }
        let key = entity.index();
        self.positions.remove(key);
        self.velocities.remove(key);
        self.healths.remove(key);
        self.armours.remove(key);
        self.entity_kinds.remove(key);
        self.block_interactions.remove(key);
        self.pathfinding.remove(key);
        true
    }

    /// Returns `true` if the entity is currently alive.
    #[inline]
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.allocator.is_alive(entity)
    }

    /// Returns the number of currently alive entities.
    #[inline]
    pub fn alive_count(&self) -> u32 {
        self.allocator.alive_count()
    }

    /// Runs a system against this world.
    pub fn run_system(&mut self, system: &impl System<MAX_ENTITIES, UNIVERSE>) {
        system.run(self);
    }
}

impl<const MAX_ENTITIES: usize, const UNIVERSE: usize> Default
    for EcsWorld<MAX_ENTITIES, UNIVERSE>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Small capacity for tests.
    type TestWorld = EcsWorld<8, 16>;

    #[test]
    fn spawn_and_add_components() {
        let mut world = TestWorld::new();

        let player = world.spawn().expect("should spawn");
        assert!(world
            .positions
            .insert(player.index(), Position::new(0.0, 64.0, 0.0))
            .is_ok());
        assert!(world
            .velocities
            .insert(player.index(), Velocity::ZERO)
            .is_ok());
        assert!(world
            .healths
            .insert(player.index(), Health::new(20.0, 20.0))
            .is_ok());
        assert!(world
            .entity_kinds
            .insert(player.index(), EntityKind::Player)
            .is_ok());

        assert_eq!(
            world.positions.get(player.index()),
            Some(&Position::new(0.0, 64.0, 0.0))
        );
        assert_eq!(
            world.entity_kinds.get(player.index()),
            Some(&EntityKind::Player)
        );
    }

    #[test]
    fn despawn_removes_components() {
        let mut world = TestWorld::new();

        let e = world.spawn().expect("should spawn");
        assert!(world
            .positions
            .insert(e.index(), Position::new(1.0, 2.0, 3.0))
            .is_ok());
        assert!(world
            .healths
            .insert(e.index(), Health::new(10.0, 10.0))
            .is_ok());

        assert!(world.despawn(e));
        assert!(!world.is_alive(e));
        assert!(world.positions.get(e.index()).is_none());
        assert!(world.healths.get(e.index()).is_none());
    }

    #[test]
    fn system_runs_on_world() {
        struct MovementSystem;

        impl System<8, 16> for MovementSystem {
            fn run(&self, world: &mut EcsWorld<8, 16>) {
                // Collect keys that have both position and velocity.
                let mut keys: heapless::Vec<u32, 8> = heapless::Vec::new();
                for &key in world.positions.keys() {
                    if world.velocities.contains(key) {
                        let _ = keys.push(key);
                    }
                }
                for key in keys {
                    if let (Some(vel), Some(pos)) =
                        (world.velocities.get(key).copied(), world.positions.get_mut(key))
                    {
                        pos.x += vel.x;
                        pos.y += vel.y;
                        pos.z += vel.z;
                    }
                }
            }
        }

        let mut world = TestWorld::new();
        let e = world.spawn().expect("should spawn");
        assert!(world
            .positions
            .insert(e.index(), Position::new(0.0, 0.0, 0.0))
            .is_ok());
        assert!(world
            .velocities
            .insert(e.index(), Velocity::new(1.0, 0.5, -1.0))
            .is_ok());

        world.run_system(&MovementSystem);

        let pos = world.positions.get(e.index()).expect("should have position");
        assert!((pos.x - 1.0).abs() < f64::EPSILON);
        assert!((pos.y - 0.5).abs() < f64::EPSILON);
        assert!((pos.z - (-1.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn spawn_entity_kinds() {
        let mut world = TestWorld::new();

        let zombie = world.spawn().expect("should spawn");
        assert!(world
            .entity_kinds
            .insert(zombie.index(), EntityKind::Zombie)
            .is_ok());
        assert!(world
            .positions
            .insert(zombie.index(), Position::new(10.0, 64.0, 10.0))
            .is_ok());
        assert!(world
            .healths
            .insert(zombie.index(), Health::new(20.0, 20.0))
            .is_ok());

        let creeper = world.spawn().expect("should spawn");
        assert!(world
            .entity_kinds
            .insert(creeper.index(), EntityKind::Creeper)
            .is_ok());
        assert!(world
            .positions
            .insert(creeper.index(), Position::new(20.0, 64.0, 20.0))
            .is_ok());
        assert!(world
            .healths
            .insert(creeper.index(), Health::new(20.0, 20.0))
            .is_ok());

        assert_eq!(world.alive_count(), 2);
        assert_eq!(
            world.entity_kinds.get(zombie.index()),
            Some(&EntityKind::Zombie)
        );
        assert_eq!(
            world.entity_kinds.get(creeper.index()),
            Some(&EntityKind::Creeper)
        );
    }

    #[test]
    fn block_interaction_workflow() {
        let mut world = TestWorld::new();

        let player = world.spawn().expect("should spawn");
        assert!(world
            .block_interactions
            .insert(player.index(), BlockInteraction::None)
            .is_ok());

        // Player starts breaking a block.
        if let Some(interaction) = world.block_interactions.get_mut(player.index()) {
            *interaction = BlockInteraction::Breaking {
                x: 10,
                y: 64,
                z: 5,
            };
        }

        assert_eq!(
            world.block_interactions.get(player.index()),
            Some(&BlockInteraction::Breaking {
                x: 10,
                y: 64,
                z: 5
            })
        );
    }
}
