use core::num::NonZeroU8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityId {
    kind: EntityKind,
    index: NonZeroU8,
}

impl EntityId {
    pub fn kind(&self) -> EntityKind {
        self.kind
    }
    pub fn index(&self) -> NonZeroU8 {
        self.index
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Player,
    Mob,
    Item,
    Projectile,
}

impl EntityKind {
    pub fn id(self) -> u8 {
        self as u8
    }
}
