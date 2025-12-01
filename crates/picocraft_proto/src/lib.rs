#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod clientbound;
pub mod game_profile;
pub mod protocol_version;
pub mod serverbound;

pub use protocol_version::{CURRENT_PROTOCOL_VERSION, CURRENT_VERSION_NAME};

#[allow(unused)]
pub mod prelude {
    pub(crate) use core::str::FromStr;

    pub(crate) use picocraft_core::byteorder::{ReadBytesExt, WriteBytesExt};
    pub(crate) use picocraft_core::prelude::*;
    pub(crate) use picocraft_derive::{Decode, Encode, Packet};

    pub use crate::game_profile::*;
    pub use crate::protocol_version::*;
    pub use crate::{clientbound, serverbound};
}
