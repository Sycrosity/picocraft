use crate::prelude::*;

impl<T, const N: usize> Array<T, N> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_vec(vec: heapless::Vec<T, N>) -> Self {
        Self(vec)
    }

    pub fn from_array(array: [T; N]) -> Self {
        Self(Vec::from_array(array))
    }
}

/// This is an anti-pattern, and probably isn't a good idea in the long run,
/// however Array is literally just a wrapper around Vec to provide
/// Encode/Decode impls for non-prefixed Arrays.
impl<T: Encode, const N: usize> core::ops::DerefMut for Array<T, N> {
    fn deref_mut(&mut self) -> &mut Vec<T, N> {
        &mut self.0
    }
}

impl<T: Encode, const N: usize> core::ops::Deref for Array<T, N> {
    type Target = Vec<T, N>;
    fn deref(&self) -> &Vec<T, N> {
        &self.0
    }
}

impl<T: Encode, const N: usize> Encode for Array<T, N> {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        for element in &self.0 {
            element.encode(&mut buffer).await?;
        }

        Ok(())
    }
}
