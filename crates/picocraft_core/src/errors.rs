use thiserror::Error;

use crate::prelude::*;

#[derive(Debug, Error)]
pub enum PicocraftError {
    #[error("unknown error")]
    Unknown,
    #[error("client couldn't connect")]
    CouldntGetClient,
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error(transparent)]
    Io(embedded_io::ErrorKind),
    #[error(transparent)]
    TryFromInt(core::num::TryFromIntError),
    #[error("This operation is unsupported")]
    UnsupportedOperation,
    #[error("unknown")]
    Unknown,
    #[error("invalid bits per entry value")]
    InvalidBPE,
}

impl<E: embedded_io::Error> From<E> for EncodeError {
    fn from(value: E) -> Self {
        EncodeError::Io(value.kind())
    }
}

impl<E: embedded_io::Error> From<embedded_io::ReadExactError<E>> for DecodeError {
    fn from(e: embedded_io::ReadExactError<E>) -> Self {
        match e {
            embedded_io::ReadExactError::UnexpectedEof => DecodeError::UnexpectedEof,
            embedded_io::ReadExactError::Other(e) => DecodeError::Io(e.kind()),
        }
    }
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error(transparent)]
    Io(#[from] embedded_io::ErrorKind),
    #[error(transparent)]
    InvalidUtf8(#[from] core::str::Utf8Error),
    #[error("an EOF error was encountered before reading the exact amount of requested bytes")]
    UnexpectedEof,
    #[error("length of VarInt did not match with the minimum size of the data structure")]
    VarIntTooSmall(VarInt),
    #[error("length of VarLong did not match with the minimum size of the data structure")]
    VarLongTooSmall(VarLong),
    #[error("cannot decode VarInt! Exceeds maximum capacity of 5 bytes.")]
    VarIntTooBig,
    #[error("cannot decode VarLong! Exceeds maximum capacity of 10 bytes.")]
    VarLongTooBig,
    #[error("custom error")]
    Custom,
    #[error("enum value is invalid")]
    InvalidEnumValue,
    #[error("Invalid boolean as the byte is not 0x01 or 0x00")]
    InvalidBoolean,
    #[error("decoding this type or variant is unsupported")]
    Unimplemented,
    #[error("no bytes should be readable when decoding [`Optional`] ")]
    UnexpectedOptionalRead,
    #[error("namespace part of Identifier is not 'minecraft'")]
    InvalidNamespace,
}
