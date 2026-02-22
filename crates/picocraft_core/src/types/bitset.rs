use crate::prelude::*;

impl<const N: usize> BitSet<N> {
    pub fn new() -> Self {
        Self(PrefixedArray::new())
    }

    pub fn from_vec(vec: heapless::Vec<Long, N>) -> Self {
        Self(PrefixedArray::from_vec(vec))
    }

    #[inline]
    pub fn from_array(array: [Long; N]) -> Self {
        Self(PrefixedArray::from_array(array))
    }
}

impl<const N: usize> From<heapless::Vec<Long, N>> for BitSet<N> {
    fn from(value: heapless::Vec<Long, N>) -> Self {
        Self(PrefixedArray::from_vec(value))
    }
}
