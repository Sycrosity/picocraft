use core::ops::{Deref, DerefMut};

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Buffer<const N: usize>(Vec<u8, N>);

/// This is an anti-pattern, and probably isn't a good idea in the long run,
/// however it greatly simplifies deriving [`Write`](embedded_io_async::Write)
/// and [`Read`](embedded_io_async::Read) for [`Vec`](heapless::Vec).
impl<const N: usize> DerefMut for Buffer<N> {
    fn deref_mut(&mut self) -> &mut Vec<u8, N> {
        &mut self.0
    }
}

impl<const N: usize> Deref for Buffer<N> {
    type Target = Vec<u8, N>;
    fn deref(&self) -> &Vec<u8, N> {
        &self.0
    }
}

impl<const N: usize> Buffer<N> {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<const N: usize> embedded_io_async::Write for Buffer<N> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        use embedded_io::Write;

        self.0.write(buf)
        // Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<const N: usize> embedded_io::ErrorType for Buffer<N> {
    type Error = heapless::CapacityError;
}
