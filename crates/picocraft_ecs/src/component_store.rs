use core::fmt::Debug;
use core::num::NonZeroU8;

use crate::prelude::*;

pub trait ComponentStore {
    type Item: ?Sized + Debug;
    fn insert(&mut self, index: u8, value: Self::Item) -> Result<(), ComponentStorageError>;
    fn remove(&mut self, index: u8) -> Result<(), ComponentStorageError>;
    fn contains(&self, index: u8) -> bool;
}

pub trait GetComponent: ComponentStore {
    fn get(&self, index: u8) -> Option<&Self::Item>;
    fn get_mut(&mut self, index: u8) -> Option<&mut Self::Item>;
}

pub struct SparseSet<T: Debug, const N: usize> {
    sparse: [Option<NonZeroU8>; N],
    dense: Vec<T, N>,
}

impl<T: Debug, const N: usize> Default for SparseSet<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug, const N: usize> SparseSet<T, N> {
    pub fn new() -> Self {
        const {
            assert!(N < (u8::MAX - 1) as usize);
        }

        Self {
            sparse: [None; N],
            dense: Vec::new(),
        }
    }
}

impl<T: Debug, const N: usize> ComponentStore for SparseSet<T, N> {
    type Item = T;

    fn insert(&mut self, index: u8, value: Self::Item) -> Result<(), ComponentStorageError> {
        if index > N as u8 {
            // Don't panic
            return Err(ComponentStorageError::IndexOutOfBounds {
                index,
                max_index: N as u8,
            });
        }

        if let Some(existing) = self.sparse[index as usize] {
            // Overwrite existing value. Maybe instead we should return a Result and let the
            // caller decide what to do?
            self.dense[existing.get() as usize - 1] = value;
        } else {
            // SAFETY - We have a const assertion that N < 255, so we can
            // safely cast the length of the dense vector to u8.
            self.sparse[index as usize] = NonZeroU8::new(self.dense.len() as u8 + 1);

            self.dense
                .push(value)
                .expect("index should never be larger than N");
        }

        Ok(())
    }

    fn remove(&mut self, index: u8) -> Result<(), ComponentStorageError> {
        if index as usize >= N {
            return Err(ComponentStorageError::IndexOutOfBounds {
                index,
                max_index: N as u8,
            });
        }

        //.take() removes the value if it exists, replacing it with `None`
        if let Some(dense_index) = self.sparse[index as usize].take() {
            // subtract 1 to get the actual index in the dense vector
            let dense_index = dense_index.get() as usize - 1;
            let last_element_index = self.dense.len() - 1;

            self.dense.swap_remove(dense_index);

            if dense_index != last_element_index {
                let moved_entity = self
                    .sparse
                    .iter()
                    .position(|sparse_element| {
                        sparse_element.map(|non_zero| non_zero.get() as usize - 1)
                            == Some(last_element_index)
                    })
                    .ok_or(ComponentStorageError::NotFound { index })?;

                self.sparse[moved_entity] = NonZeroU8::new(dense_index as u8 + 1);
            }
        } else {
            return Err(ComponentStorageError::NotFound { index });
        }

        Ok(())
    }

    fn contains(&self, index: u8) -> bool {
        self.sparse[index as usize].is_some()
    }
}

impl<T: Debug, const N: usize> GetComponent for SparseSet<T, N> {
    fn get(&self, index: u8) -> Option<&Self::Item> {
        let dense_index = self.sparse[index as usize]?.get() as usize - 1;

        Some(&self.dense[dense_index])
    }

    fn get_mut(&mut self, index: u8) -> Option<&mut Self::Item> {
        let dense_index = self.sparse[index as usize]?.get() as usize - 1;

        Some(&mut self.dense[dense_index])
    }
}

/// A SparseSet for components that only store Zero-Sized Types (e.g. OnGround,
/// IsTouchingWall, etc.)
pub struct MarkerSet<T: Debug, const N: usize> {
    _marker: core::marker::PhantomData<T>,
    bits: [u128; 2],
}

impl<T: Debug, const N: usize> MarkerSet<T, N> {
    pub fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
            bits: [0; 2],
        }
    }

    pub fn mark(&mut self, index: u8) {
        self.bits[(index / 128) as usize] |= 1 << (index % 128);
    }

    pub fn unmark(&mut self, index: u8) {
        self.bits[(index / 128) as usize] &= !(1 << (index % 128));
    }

    pub fn contains(&self, index: u8) -> bool {
        (self.bits[(index / 128) as usize] & (1 << (index % 128))) != 0
    }
}

impl<T: Debug, const N: usize> Default for MarkerSet<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug, const N: usize> ComponentStore for MarkerSet<T, N> {
    type Item = T;

    fn insert(&mut self, index: u8, _value: Self::Item) -> Result<(), ComponentStorageError> {
        if index > N as u8 {
            return Err(ComponentStorageError::IndexOutOfBounds {
                index,
                max_index: N as u8,
            });
        }
        self.mark(index);
        Ok(())
    }

    fn remove(&mut self, index: u8) -> Result<(), ComponentStorageError> {
        if index > N as u8 {
            return Err(ComponentStorageError::IndexOutOfBounds {
                index,
                max_index: N as u8,
            });
        }
        self.unmark(index);
        Ok(())
    }

    fn contains(&self, index: u8) -> bool {
        self.contains(index)
    }
}
