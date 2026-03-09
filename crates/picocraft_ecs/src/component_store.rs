use core::num::NonZeroU8;

use crate::entity::EntityId;
use crate::prelude::*;

trait ComponentStore {
    type Item: ?Sized;
    fn insert(&mut self, index: NonZeroU8, value: Self::Item);
    fn remove(&mut self, index: NonZeroU8);
    fn get(&self, index: NonZeroU8) -> Option<&Self::Item>;
    fn get_mut(&mut self, index: NonZeroU8) -> Option<&mut Self::Item>;
    fn contains(&self, index: NonZeroU8) -> bool;
}

pub struct SparseSet<T, const N: usize> {
    sparse: [Option<NonZeroU8>; N],
    dense: Vec<T, N>,
}

impl<T, const N: usize> Default for SparseSet<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> SparseSet<T, N> {
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

impl<T, const N: usize> ComponentStore for SparseSet<T, N> {
    type Item = T;

    fn insert(&mut self, index: NonZeroU8, value: Self::Item) {
        todo!()
    }

    fn remove(&mut self, index: NonZeroU8) {
        todo!()
    }

    fn get(&self, index: NonZeroU8) -> Option<&Self::Item> {
        todo!()
    }

    fn get_mut(&mut self, index: NonZeroU8) -> Option<&mut Self::Item> {
        todo!()
    }

    fn contains(&self, index: NonZeroU8) -> bool {
        todo!()
    }
}

/// A SparseSet for components that only store Zero-Sized Types (e.g. OnGround,
/// IsTouchingWall, etc.)
pub struct MarkerSet<T, const N: usize> {
    marker_type: core::marker::PhantomData<T>,
    sparse: [Option<NonZeroU8>; N],
}

impl<T, const N: usize> MarkerSet<T, N> {
    pub fn new() -> Self {
        Self {
            marker_type: core::marker::PhantomData,
            sparse: [None; N],
        }
    }
}

impl<T, const N: usize> Default for MarkerSet<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> ComponentStore for MarkerSet<T, N> {
    type Item = T;

    fn insert(&mut self, _index: NonZeroU8, _value: Self::Item) {
        todo!()
    }

    fn remove(&mut self, _index: NonZeroU8) {
        todo!()
    }

    fn get(&self, _index: NonZeroU8) -> Option<&Self::Item> {
        todo!()
    }

    fn get_mut(&mut self, _index: NonZeroU8) -> Option<&mut Self::Item> {
        todo!()
    }

    fn contains(&self, _index: NonZeroU8) -> bool {
        todo!()
    }
}
