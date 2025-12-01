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

//     pub fn inner(&self) -> &Vec<u8, N> {
//         &self.0
//     }

//     pub fn len(&self) -> usize {
//         self.0.len()
//     }

//     pub fn is_empty(&self) -> bool {
//         self.0.is_empty()
//     }

//     pub fn clear(&mut self) {
//         self.0.clear();
//     }

//     pub fn as_slice(&self) -> &[u8] {
//         self.0.as_slice()
//     }

//     pub fn into_inner(self) -> Vec<u8, N> {
//         self.0
//     }

//     pub fn extend_from_slice(&mut self, slice: &[u8]) -> Result<(),
// heapless::CapacityError> {         self.0.extend_from_slice(slice)
//     }

// }

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

// impl<const N: usize> embedded_io::Error for Buffer<N> {
//     fn kind(&self) -> embedded_io::ErrorKind {
//         embedded_io::ErrorKind::OutOfMemory
//     }
// }

// impl<T: Decode, const N: usize> Decode for Vec<T, N> {
//     async fn decode(mut buffer: impl Read) -> Result<Self> {
//         // let length = VarInt::decode(&mut buffer)?.0;

//         // ensure!(
//         //     length >= 0,
//         //     "Attempted to decode struct with negative length"
//         // );

//         // Don't allocate more memory than what would roughly fit in a single
// packet in         // case we get a malicious array length.
//         // let cap = (*MAX_PACKET_SIZE as usize /
// std::mem::size_of::<T>().max(1)).min(length as usize);         // let mut vec
// = Self::with_capacity(cap);

//         let mut vec = Vec::new();

//         for _ in 0..length {
//             vec.push(T::decode(&mut buffer)?);
//         }

//         Ok(vec)
//     }
// }

// impl<T: Encode> Encode for Option<T> {
//     fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) ->
// Result<(), EncodeError<W::Error>> {         match self {
//             Some(t) => {
//                 true.encode(&mut buffer)?;
//                 t.encode(buffer)
//             }
//             None => false.encode(&mut buffer),
//         }
//     }
// }

// impl<T: Decode> Decode for Option<T> {
//     fn decode(mut buffer: impl Read) -> Result<Self> {
//         Ok(match bool::decode(&mut buffer)? {
//             true => Some(T::decode(&mut buffer)?),
//             false => None,
//         })
//     }
// }

// impl Decode for Uuid {
//     fn decode(buffer: impl Read) -> Result<Self> {
//         u128::decode(buffer).map(Uuid::from_u128)
//     }
// }
