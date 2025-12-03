use embedded_io_async::{Read, ReadExactError, Write};
use thiserror::Error;

use crate::prelude::*;

pub const MAX_PACKET_SIZE: VarInt = VarInt(2_097_152);

/// \[`VarInt`, `VarInt`, `[u8]`\]
pub trait Packet: Decode + Encode + Sized + core::fmt::Debug {
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
    async fn encode<W: Write>(&self, buffer: W) -> Result<(), EncodeError<W::Error>>;
}

#[allow(async_fn_in_trait)]
pub trait Decode: Sized {
    async fn decode<R: Read>(buffer: R) -> Result<Self, DecodeError<R::Error>>;
}

#[derive(Debug, Error)]
pub enum EncodeError<E: embedded_io_async::Error> {
    #[error(transparent)]
    Io(#[from] E),
    #[error(transparent)]
    TryFromInt(core::num::TryFromIntError),
    #[error("This operation is unsupported")]
    UnsupportedOperation,
}

#[derive(Debug, Error)]
pub enum DecodeError<E: embedded_io_async::Error> {
    #[error(transparent)]
    Io(#[from] E),
    #[error(transparent)]
    InvalidUtf8(core::str::Utf8Error),
    #[error("an EOF error was encountered before reading the exact amount of requested bytes")]
    UnexpectedEof,
    #[error("length of VarInt did not match with the minimum size of the data structure")]
    VarIntTooSmall(VarInt),
    #[error(
        "cannot decode VarInt! Exceeds maximum capacity of 5 bytes. Max/Min: \
         (2147483647/-2147483648)"
    )]
    VarIntTooBig,
    #[error("custom error")]
    Custom,
    #[error("enum value is invalid")]
    InvalidEnumValue,
    #[error("Invalid boolean as the byte is not 0x01 or 0x00")]
    InvalidBoolean,
    #[error("no bytes should be readable when decoding [`Optional`] ")]
    UnexpectedOptionalRead,
    #[error("namespace part of Identifier is not 'minecraft'")]
    InvalidNamespace,
}

impl<E: embedded_io::Error> From<ReadExactError<E>> for DecodeError<E> {
    fn from(value: ReadExactError<E>) -> Self {
        match value {
            ReadExactError::UnexpectedEof => Self::UnexpectedEof,
            ReadExactError::Other(e) => Self::Io(e),
        }
    }
}

macro_rules! impl_decode_integer {
    ($($ty:ty), *) => {
        $(
        impl crate::packet::Decode for $ty {

            async fn decode<R>(mut buffer: R) -> ::core::result::Result<Self, crate::packet::DecodeError<R::Error>>
            where R: ::embedded_io_async::Read
            {
                const SIZE: usize = core::mem::size_of::<$ty>();

                let mut buf = [0; SIZE];

                buffer.read_exact(&mut buf).await?;
                Ok(<$ty>::from_be_bytes(buf[..SIZE].try_into().expect("decoding integers should always work")))
            }
        }
        )*
    };
}

impl_decode_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128);

macro_rules! impl_encode_integer {
    ($($ty:ty), *) => {
        $(
        impl crate::packet::Encode for $ty {

            async fn encode<W>(&self, mut buffer: W) -> ::core::result::Result<(), crate::packet::EncodeError<W::Error>>
            where W: ::embedded_io_async::Write
            {
                buffer.write_all(&self.to_be_bytes()).await?;
                Ok(())
            }

        }
        )*
    };
}

impl_encode_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128);
