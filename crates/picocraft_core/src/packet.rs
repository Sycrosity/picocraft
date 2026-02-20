use crate::prelude::*;

pub const MAX_PACKET_SIZE: VarInt = VarInt(2_097_152);

/// \[`VarInt`, `VarInt`, `[u8]`\]
pub trait Packet: Decode + Encode + Sized + core::fmt::Debug + core::fmt::Display {
    const ID: VarInt;
    const STATE: State;

    /// the [`VarInt`] ID of a specified packet (needed to send
    /// any type of any packet)
    fn id(&self) -> VarInt {
        Self::ID
    }
    /// which server [`State`] this packet is a part of.
    fn state(&self) -> State {
        Self::STATE
    }
}

#[allow(async_fn_in_trait)]
pub trait Encode: Sized {
    /// Writes this object to the provided writer.
    ///
    /// If this type also implements [`Decode`] then successful calls to this
    /// function returning `Ok(())` must always successfully [`Decode::decode`]
    /// using the data that was written to the writer. The exact number of
    /// bytes that were originally written must be consumed during the
    /// decoding.
    async fn encode<W: Write>(&self, buffer: W) -> Result<(), EncodeError>;
}

#[allow(async_fn_in_trait)]
pub trait Decode: Sized {
    async fn decode<R: Read>(buffer: R) -> Result<Self, DecodeError>;
}

#[derive(Debug, Default)]
pub struct ByteCountWriter {
    pub count: usize,
}

impl ByteCountWriter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Write for ByteCountWriter {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.count += buf.len();
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl embedded_io::ErrorType for ByteCountWriter {
    type Error = core::convert::Infallible;
}
