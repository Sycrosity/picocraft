use crate::prelude::*;

impl<T, const N: usize> PrefixedArray<T, N> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_vec(vec: heapless::Vec<T, N>) -> Self {
        Self(vec)
    }

    pub fn from_array<const M: usize>(array: [T; M]) -> Self {
        Self(Vec::from_array(array))
    }
}

/// This is an anti-pattern, and probably isn't a good idea in the long run,
/// however Array is literally just a wrapper around Vec to provide
/// Encode/Decode impls for non-prefixed Arrays.
impl<T: Encode, const N: usize> core::ops::DerefMut for PrefixedArray<T, N> {
    fn deref_mut(&mut self) -> &mut Vec<T, N> {
        &mut self.0
    }
}

impl<T: Encode, const N: usize> core::ops::Deref for PrefixedArray<T, N> {
    type Target = Vec<T, N>;
    fn deref(&self) -> &Vec<T, N> {
        &self.0
    }
}

impl<T: Encode, const N: usize> Encode for PrefixedArray<T, N> {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        VarInt(self.len() as i32).encode(&mut buffer).await?;

        for element in &self.0 {
            element.encode(&mut buffer).await?;
        }

        Ok(())
    }
}

impl<T: Decode, const N: usize> Decode for PrefixedArray<T, N> {
    async fn decode<R: embedded_io_async::Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let length = *VarInt::decode(&mut buffer).await?;

        if !length.is_positive() {
            return Err(DecodeError::VarIntTooSmall(VarInt(0)));
        }

        let mut vec = Self::new();

        for _ in 0..length {
            let _ = vec.0.push(T::decode(&mut buffer).await?);
        }

        Ok(vec)
    }
}

impl<T: Encode + Decode, const N: usize> From<heapless::Vec<T, N>> for PrefixedArray<T, N> {
    fn from(value: heapless::Vec<T, N>) -> Self {
        Self(value)
    }
}
