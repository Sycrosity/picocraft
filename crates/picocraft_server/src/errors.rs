use thiserror::Error;

use crate::prelude::*;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Invalid packet")]
    InvalidPacket,
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("Unknown error")]
    Unknown,
    #[error(transparent)]
    SocketError(#[from] SocketError),
    #[error("Encode error")]
    EncodeError,
    #[error("Decode error")]
    DecodeError,
}

impl<E: embedded_io::Error> From<EncodeError<E>> for PacketError {
    fn from(error: EncodeError<E>) -> Self {
        warn!("{error:?}");

        PacketError::EncodeError
    }
}

impl<E: embedded_io::Error> From<DecodeError<E>> for PacketError {
    fn from(error: DecodeError<E>) -> Self {
        warn!("{error:?}");

        PacketError::DecodeError
    }
}

#[derive(Debug, Error, Clone, Copy)]
pub enum SocketError {
    #[error("IO")]
    IoError,
    #[error("Unexpected Eof")]
    UnexpectedEof,
    #[error("socket is not (yet) readable")]
    NotReadable,
    #[error("tried to read/write an oversized buffer")]
    OversizedBuffer,
}

impl From<embedded_io::ReadExactError<SocketError>> for SocketError {
    fn from(e: embedded_io::ReadExactError<SocketError>) -> Self {
        match e {
            embedded_io::ReadExactError::UnexpectedEof => SocketError::UnexpectedEof,
            embedded_io::ReadExactError::Other(e) => e,
        }
    }
}

impl embedded_io::Error for SocketError {
    fn kind(&self) -> embedded_io::ErrorKind {
        #[allow(clippy::match_single_binding)]
        match self {
            _ => embedded_io::ErrorKind::Other,
        }
    }
}
