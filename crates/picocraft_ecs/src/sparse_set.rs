/// A stack-allocated sparse set mapping `u32` keys to values of type `T`.
///
/// `CAPACITY` is the maximum number of entries (dense storage), and
/// `UNIVERSE` is the range of valid keys (`0..UNIVERSE`), which determines
/// the size of the sparse index array.
///
/// This gives O(1) insert, remove, get, and contains operations, with
/// iteration over the dense array for cache-friendly component access.
pub struct SparseSet<T, const CAPACITY: usize, const UNIVERSE: usize> {
    /// Maps key → position in the dense array. `u32::MAX` means empty.
    sparse: [u32; UNIVERSE],
    /// Packed keys.
    dense_keys: heapless::Vec<u32, CAPACITY>,
    /// Packed values, parallel to `dense_keys`.
    dense_values: heapless::Vec<T, CAPACITY>,
}

impl<T, const CAPACITY: usize, const UNIVERSE: usize> SparseSet<T, CAPACITY, UNIVERSE> {
    const EMPTY: u32 = u32::MAX;

    /// Creates a new empty sparse set.
    pub fn new() -> Self {
        Self {
            sparse: [Self::EMPTY; UNIVERSE],
            dense_keys: heapless::Vec::new(),
            dense_values: heapless::Vec::new(),
        }
    }

    /// Returns the number of entries in the set.
    #[inline]
    pub fn len(&self) -> usize {
        self.dense_keys.len()
    }

    /// Returns `true` if the set contains no entries.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.dense_keys.is_empty()
    }

    /// Returns `true` if the set contains a value for the given key.
    #[inline]
    pub fn contains(&self, key: u32) -> bool {
        let k = key as usize;
        k < UNIVERSE && self.sparse[k] != Self::EMPTY
    }

    /// Inserts a value for the given key.
    ///
    /// Returns `Ok(())` on success, `Err(value)` if either the key is out of
    /// range or the set is at capacity. If the key already exists, the value
    /// is updated in place and the old value is dropped.
    pub fn insert(&mut self, key: u32, value: T) -> Result<(), T> {
        let k = key as usize;
        if k >= UNIVERSE {
            return Err(value);
        }
        if self.sparse[k] != Self::EMPTY {
            // Update existing entry.
            let dense_idx = self.sparse[k] as usize;
            self.dense_values[dense_idx] = value;
            return Ok(());
        }
        let dense_idx = self.dense_keys.len() as u32;
        if self.dense_keys.push(key).is_err() {
            return Err(value);
        }
        if let Err(value) = self.dense_values.push(value) {
            // Keep consistent — undo the key push. This path is unreachable
            // when both vecs share the same CAPACITY, but handled for safety.
            self.dense_keys.pop();
            return Err(value);
        }
        self.sparse[k] = dense_idx;
        Ok(())
    }

    /// Removes the value for the given key, returning it if present.
    pub fn remove(&mut self, key: u32) -> Option<T> {
        let k = key as usize;
        if k >= UNIVERSE || self.sparse[k] == Self::EMPTY {
            return None;
        }
        let dense_idx = self.sparse[k] as usize;
        self.sparse[k] = Self::EMPTY;

        let last_idx = self.dense_keys.len() - 1;

        // Swap-remove from both parallel arrays.
        self.dense_keys.swap(dense_idx, last_idx);
        self.dense_values.swap(dense_idx, last_idx);

        self.dense_keys.pop();
        let value = self.dense_values.pop().expect("dense_values should not be empty");

        // Update the sparse entry for the element that was swapped in.
        if dense_idx < self.dense_keys.len() {
            let swapped_key = self.dense_keys[dense_idx] as usize;
            self.sparse[swapped_key] = dense_idx as u32;
        }

        Some(value)
    }

    /// Returns a reference to the value for the given key, if present.
    #[inline]
    pub fn get(&self, key: u32) -> Option<&T> {
        let k = key as usize;
        if k >= UNIVERSE || self.sparse[k] == Self::EMPTY {
            return None;
        }
        Some(&self.dense_values[self.sparse[k] as usize])
    }

    /// Returns a mutable reference to the value for the given key, if present.
    #[inline]
    pub fn get_mut(&mut self, key: u32) -> Option<&mut T> {
        let k = key as usize;
        if k >= UNIVERSE || self.sparse[k] == Self::EMPTY {
            return None;
        }
        Some(&mut self.dense_values[self.sparse[k] as usize])
    }

    /// Returns a slice of all keys in dense (packed) order.
    #[inline]
    pub fn keys(&self) -> &[u32] {
        &self.dense_keys
    }

    /// Returns a slice of all values in dense (packed) order.
    #[inline]
    pub fn values(&self) -> &[T] {
        &self.dense_values
    }

    /// Returns a mutable slice of all values in dense (packed) order.
    #[inline]
    pub fn values_mut(&mut self) -> &mut [T] {
        &mut self.dense_values
    }

    /// Returns an iterator over `(key, &value)` pairs in dense order.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (u32, &T)> {
        self.dense_keys
            .iter()
            .copied()
            .zip(self.dense_values.iter())
    }

    /// Returns a mutable iterator over `(key, &mut value)` pairs in dense
    /// order.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut T)> {
        self.dense_keys
            .iter()
            .copied()
            .zip(self.dense_values.iter_mut())
    }

    /// Removes all entries.
    pub fn clear(&mut self) {
        for &key in self.dense_keys.iter() {
            self.sparse[key as usize] = Self::EMPTY;
        }
        self.dense_keys.clear();
        self.dense_values.clear();
    }
}

impl<T, const CAPACITY: usize, const UNIVERSE: usize> Default
    for SparseSet<T, CAPACITY, UNIVERSE>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut set = SparseSet::<i32, 8, 16>::new();
        assert!(set.insert(3, 100).is_ok());
        assert!(set.insert(7, 200).is_ok());
        assert_eq!(set.get(3), Some(&100));
        assert_eq!(set.get(7), Some(&200));
        assert_eq!(set.get(0), None);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn insert_overwrites() {
        let mut set = SparseSet::<i32, 4, 8>::new();
        assert!(set.insert(2, 10).is_ok());
        assert!(set.insert(2, 20).is_ok());
        assert_eq!(set.get(2), Some(&20));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn remove_and_swap() {
        let mut set = SparseSet::<&str, 4, 8>::new();
        assert!(set.insert(1, "a").is_ok());
        assert!(set.insert(3, "b").is_ok());
        assert!(set.insert(5, "c").is_ok());

        assert_eq!(set.remove(1), Some("a"));
        assert!(!set.contains(1));
        // The last element (5) was swapped into position 0.
        assert_eq!(set.get(5), Some(&"c"));
        assert_eq!(set.get(3), Some(&"b"));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn out_of_range_key() {
        let mut set = SparseSet::<i32, 4, 4>::new();
        assert!(set.insert(4, 1).is_err()); // key == UNIVERSE, out of range
        assert!(set.insert(100, 1).is_err());
    }

    #[test]
    fn capacity_limit() {
        let mut set = SparseSet::<i32, 2, 8>::new();
        assert!(set.insert(0, 10).is_ok());
        assert!(set.insert(1, 20).is_ok());
        assert!(set.insert(2, 30).is_err()); // at capacity
    }

    #[test]
    fn clear() {
        let mut set = SparseSet::<i32, 4, 8>::new();
        assert!(set.insert(1, 10).is_ok());
        assert!(set.insert(3, 30).is_ok());
        set.clear();
        assert!(set.is_empty());
        assert!(!set.contains(1));
        assert!(!set.contains(3));
    }

    #[test]
    fn iteration() {
        let mut set = SparseSet::<i32, 4, 8>::new();
        assert!(set.insert(2, 20).is_ok());
        assert!(set.insert(4, 40).is_ok());

        let mut pairs: heapless::Vec<(u32, i32), 4> = heapless::Vec::new();
        for (k, v) in set.iter() {
            let _ = pairs.push((k, *v));
        }
        pairs.sort_unstable();
        assert_eq!(pairs.as_slice(), &[(2, 20), (4, 40)]);
    }

    #[test]
    fn mutation() {
        let mut set = SparseSet::<i32, 4, 8>::new();
        assert!(set.insert(1, 10).is_ok());
        if let Some(v) = set.get_mut(1) {
            *v = 99;
        }
        assert_eq!(set.get(1), Some(&99));
    }
}
