#![cfg_attr(not(test), no_std)]
#![warn(
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::large_include_file,
    clippy::panic_in_result_fn,
    clippy::redundant_test_prefix,
    clippy::std_instead_of_core,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unwrap_used
)]

#[cfg(feature = "std")]
extern crate std;

pub mod chunks;
pub mod clientbound;
pub mod game_profile;
pub mod protocol_version;
pub mod serverbound;

pub use protocol_version::{CURRENT_PROTOCOL_VERSION, CURRENT_VERSION_NAME};

#[allow(unused)]
pub mod prelude {

    pub(crate) use heapless::Vec;
    pub(crate) use picocraft_core::byteorder::{ReadBytesExt, WriteBytesExt};
    pub(crate) use picocraft_core::prelude::*;
    pub(crate) use picocraft_derive::{Decode, Encode, Packet};

    pub(crate) use crate::chunks::{ChunkDataProto, LightDataProto};
    pub use crate::game_profile::*;
    pub use crate::protocol_version::*;
    pub use crate::{clientbound, serverbound};
}
