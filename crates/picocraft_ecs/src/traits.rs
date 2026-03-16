use crate::entity::{EntityId, EntityRef};
use crate::prelude::*;

pub trait InsertInto<Pool> {
    /// Inserts this component into the given pool at the specified index.
    fn insert_into(self, pool: &mut Pool, index: u8) -> Result<(), ComponentStorageError>;
}

pub trait RemoveFrom<Pool> {
    fn remove_from(pool: &mut Pool, index: u8) -> Result<(), ComponentStorageError>;
}

pub trait Pool: Sized {
    type Bundle;
    type SaveData;

    fn spawn(&mut self, bundle: Self::Bundle)
    -> Result<EntityRef<'_, Self>, ComponentStorageError>;
    fn despawn(&mut self, entity_id: EntityId) -> Result<(), ComponentStorageError>;
}
