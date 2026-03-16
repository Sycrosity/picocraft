use core::num::NonZeroU8;

use crate::prelude::*;
use crate::traits::{InsertInto, RemoveFrom};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityId {
    kind: EntityKind,
    index: NonZeroU8,
}

impl EntityId {
    pub fn kind(&self) -> EntityKind {
        self.kind
    }
    pub fn index(&self) -> u8 {
        self.index.get() - 1
    }

    pub fn protocol_id(&self) -> VarInt {
        // in case I change how EntityId works in the future
        let size_of_index = size_of_val(&self.index());
        VarInt(((self.kind.id() as i32) << size_of_index) | i32::from(self.index()))
    }

    pub fn new(kind: EntityKind, index: u8) -> Self {
        assert!(index < u8::MAX, "Entity index must be less than 255");

        Self {
            kind,
            index: NonZeroU8::new(index + 1).expect("Entity index should be non-zero"),
        }
    }

    pub fn player(index: u8) -> Self {
        Self::new(EntityKind::Player, index)
    }

    pub fn mob(index: u8) -> Self {
        Self::new(EntityKind::Mob, index)
    }

    pub fn item(index: u8) -> Self {
        Self::new(EntityKind::Item, index)
    }

    pub fn projectile(index: u8) -> Self {
        Self::new(EntityKind::Projectile, index)
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

pub struct EntityRef<'a, Pool> {
    pub pool: &'a mut Pool,
    pub entity_id: EntityId,
}

impl<'a, Pool> EntityRef<'a, Pool> {
    pub fn insert<C: InsertInto<Pool>>(
        &mut self,
        component: C,
    ) -> Result<(), ComponentStorageError> {
        component.insert_into(self.pool, self.entity_id.index())
    }

    pub fn remove<C: RemoveFrom<Pool>>(&mut self) -> Result<(), ComponentStorageError> {
        C::remove_from(self.pool, self.entity_id.index())
    }
}
