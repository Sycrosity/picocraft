#[derive(Debug, thiserror::Error)]
pub enum ComponentStorageError {
    #[error("entity at index {index} is out of bounds (max index: {max_index})")]
    IndexOutOfBounds { index: u8, max_index: u8 },
    #[error("entity at index {0} not found")]
    NotFound(u8),
}
