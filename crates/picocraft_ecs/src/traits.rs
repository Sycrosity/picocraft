use crate::prelude::*;

pub trait InsertInto<Pool> {
    /// Inserts this component into the given pool at the specified index.
    fn insert_into(self, pool: &mut Pool, index: u8) -> Result<(), ComponentStorageError>;
}

pub trait RemoveFrom<Pool> {
    fn remove_from(pool: &mut Pool, index: u8) -> Result<(), ComponentStorageError>;
}
