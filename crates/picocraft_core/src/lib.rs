#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate std;

/// `embedded_io_async` compatible versions of basic `byteorder` traits.
pub mod byteorder;
pub mod errors;
pub mod packet;
pub mod state;
pub mod types;

pub use crate::types::{UUID, VarInt};

pub mod prelude {
    pub use crate::errors::*;
    pub use crate::packet::*;
    pub use crate::state::*;
    pub use crate::types::*;
}
