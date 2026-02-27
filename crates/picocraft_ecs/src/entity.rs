/// A unique identifier for an entity in the ECS world.
///
/// Entities are composed of a 24-bit index and an 8-bit generation counter,
/// packed into a single `u32`. The generation prevents stale references to
/// recycled entity slots.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Entity(u32);

impl Entity {
    const INDEX_BITS: u32 = 24;
    const INDEX_MASK: u32 = (1 << Self::INDEX_BITS) - 1;
    const GENERATION_SHIFT: u32 = Self::INDEX_BITS;

    /// Creates a new `Entity` from an index and generation.
    #[inline]
    pub const fn new(index: u32, generation: u8) -> Self {
        Self((index & Self::INDEX_MASK) | ((generation as u32) << Self::GENERATION_SHIFT))
    }

    /// Returns the index portion of the entity id.
    #[inline]
    pub const fn index(self) -> u32 {
        self.0 & Self::INDEX_MASK
    }

    /// Returns the generation portion of the entity id.
    #[inline]
    pub const fn generation(self) -> u8 {
        (self.0 >> Self::GENERATION_SHIFT) as u8
    }
}

impl core::fmt::Debug for Entity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Entity({}v{})", self.index(), self.generation())
    }
}

/// Allocates and recycles entity identifiers.
///
/// `MAX_ENTITIES` is the compile-time upper bound on the number of
/// simultaneously alive entities. Entity indices range from `0` to
/// `MAX_ENTITIES - 1`; these indices are then used as keys into sparse
/// sets whose `UNIVERSE` parameter must be `>= MAX_ENTITIES`.
pub struct EntityAllocator<const MAX_ENTITIES: usize> {
    /// Generations for each entity slot. Incremented on deallocation.
    generations: [u8; MAX_ENTITIES],
    /// Stack of free entity indices available for reuse.
    free_list: heapless::Vec<u32, MAX_ENTITIES>,
    /// The next fresh index that has never been used.
    next_index: u32,
    /// The number of currently alive entities.
    alive: u32,
}

impl<const MAX_ENTITIES: usize> EntityAllocator<MAX_ENTITIES> {
    /// Creates a new empty allocator.
    pub fn new() -> Self {
        Self {
            generations: [0u8; MAX_ENTITIES],
            free_list: heapless::Vec::new(),
            next_index: 0,
            alive: 0,
        }
    }

    /// Allocates a new entity, returning `None` if the maximum capacity has
    /// been reached.
    pub fn allocate(&mut self) -> Option<Entity> {
        if let Some(index) = self.free_list.pop() {
            let generation = self.generations[index as usize];
            self.alive += 1;
            Some(Entity::new(index, generation))
        } else if (self.next_index as usize) < MAX_ENTITIES {
            let index = self.next_index;
            self.next_index += 1;
            self.alive += 1;
            Some(Entity::new(index, 0))
        } else {
            None
        }
    }

    /// Deallocates an entity, making its index available for reuse.
    ///
    /// Returns `true` if the entity was alive and is now deallocated.
    pub fn deallocate(&mut self, entity: Entity) -> bool {
        let idx = entity.index() as usize;
        if idx >= MAX_ENTITIES {
            return false;
        }
        if self.generations[idx] != entity.generation() {
            return false;
        }
        self.generations[idx] = self.generations[idx].wrapping_add(1);
        // Capacity is always sufficient: an index can only be freed once before
        // it is reallocated, and the free list never exceeds MAX_ENTITIES.
        let _ = self.free_list.push(entity.index());
        self.alive -= 1;
        true
    }

    /// Returns `true` if the entity is currently alive.
    pub fn is_alive(&self, entity: Entity) -> bool {
        let idx = entity.index() as usize;
        idx < MAX_ENTITIES && self.generations[idx] == entity.generation()
    }

    /// Returns the number of currently alive entities.
    pub fn alive_count(&self) -> u32 {
        self.alive
    }
}

impl<const MAX_ENTITIES: usize> Default for EntityAllocator<MAX_ENTITIES> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_index_and_generation() {
        let e = Entity::new(42, 3);
        assert_eq!(e.index(), 42);
        assert_eq!(e.generation(), 3);
    }

    #[test]
    fn allocate_and_deallocate() {
        let mut alloc = EntityAllocator::<4>::new();

        let e0 = alloc.allocate().expect("should allocate");
        let e1 = alloc.allocate().expect("should allocate");
        assert_eq!(alloc.alive_count(), 2);
        assert!(alloc.is_alive(e0));
        assert!(alloc.is_alive(e1));

        assert!(alloc.deallocate(e0));
        assert!(!alloc.is_alive(e0));
        assert_eq!(alloc.alive_count(), 1);

        // Recycled slot gets incremented generation.
        let e2 = alloc.allocate().expect("should allocate recycled slot");
        assert_eq!(e2.index(), e0.index());
        assert_eq!(e2.generation(), 1);
        assert!(alloc.is_alive(e2));
    }

    #[test]
    fn capacity_limit() {
        let mut alloc = EntityAllocator::<2>::new();
        assert!(alloc.allocate().is_some());
        assert!(alloc.allocate().is_some());
        assert!(alloc.allocate().is_none());
    }

    #[test]
    fn double_deallocate_returns_false() {
        let mut alloc = EntityAllocator::<4>::new();
        let e = alloc.allocate().expect("should allocate");
        assert!(alloc.deallocate(e));
        assert!(!alloc.deallocate(e));
    }
}
