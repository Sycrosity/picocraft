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
    Socket(#[from] SocketError),
    #[error(transparent)]
    Encode(#[from] EncodeError),
    #[error(transparent)]
    Decode(#[from] DecodeError),
}

#[derive(Debug, Error, Clone, Copy)]
pub enum SocketError {
    #[error(transparent)]
    Io(#[from] embedded_io::ErrorKind),
    #[error("Unexpected Eof")]
    UnexpectedEof,
    #[error("socket is not (yet) readable")]
    NotReadable,
    #[error("tried to read/write an oversized buffer")]
    OversizedBuffer,
}

#[cfg(feature = "std")]
impl From<std::io::Error> for SocketError {
    fn from(e: std::io::Error) -> Self {
        SocketError::Io(e.kind().into())
    }
}

impl From<embedded_io::ReadExactError<SocketError>> for SocketError {
    fn from(e: embedded_io::ReadExactError<SocketError>) -> Self {
        match e {
            embedded_io::ReadExactError::UnexpectedEof => SocketError::UnexpectedEof,
            embedded_io::ReadExactError::Other(e) => e,
        }
    }
}

impl<E: embedded_io::Error> From<embedded_io::ReadExactError<E>> for PacketError {
    fn from(e: embedded_io::ReadExactError<E>) -> Self {
        match e {
            embedded_io::ReadExactError::UnexpectedEof => PacketError::ConnectionClosed,
            embedded_io::ReadExactError::Other(e) => PacketError::Socket(SocketError::Io(e.kind())),
        }
    }
}

// impl From<SocketError> for EncodeError {
//     fn from(value: SocketError) -> Self {
//         match value {
//             SocketError::Io(e) => EncodeError::Io(e),
//             SocketError::UnexpectedEof => EncodeError::Unknown,
//             SocketError::NotReadable => EncodeError::UnsupportedOperation,
//             SocketError::OversizedBuffer => EncodeError::Unknown,
//         }
//     }
// }

impl embedded_io::Error for SocketError {
    fn kind(&self) -> embedded_io::ErrorKind {
        #[allow(clippy::match_single_binding)]
        match self {
            Self::Io(kind) => *kind,
            Self::UnexpectedEof => embedded_io::ErrorKind::TimedOut,
            Self::NotReadable => embedded_io::ErrorKind::Unsupported,
            Self::OversizedBuffer => embedded_io::ErrorKind::OutOfMemory,
        }
    }
}
