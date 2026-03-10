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
    pub fn index(&self) -> NonZeroU8 {
        self.index
    }

    pub fn player(index: u8) -> Self {
        Self {
            kind: EntityKind::Player,
            index: NonZeroU8::new(index).expect("Entity index must be non-zero"),
        }
    }

    pub fn mob(index: u8) -> Self {
        Self {
            kind: EntityKind::Mob,
            index: NonZeroU8::new(index).expect("Entity index must be non-zero"),
        }
    }

    pub fn item(index: u8) -> Self {
        Self {
            kind: EntityKind::Item,
            index: NonZeroU8::new(index).expect("Entity index must be non-zero"),
        }
    }

    pub fn projectile(index: u8) -> Self {
        Self {
            kind: EntityKind::Projectile,
            index: NonZeroU8::new(index).expect("Entity index must be non-zero"),
        }
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
    pub index: u8,
}

impl<'a, Pool> EntityRef<'a, Pool> {
    pub fn insert<C: InsertInto<Pool>>(
        &mut self,
        component: C,
    ) -> Result<(), ComponentStorageError> {
        component.insert_into(self.pool, self.index)
    }

    pub fn remove<C: RemoveFrom<Pool>>(&mut self) -> Result<(), ComponentStorageError> {
        C::remove_from(self.pool, self.index)
    }
}
