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
    pub(crate) use embedded_io_async::{Read, Write};

    pub(crate) use crate::byteorder::{ReadBytesExt, WriteBytesExt};
    pub use crate::errors::*;
    pub use crate::packet::*;
    pub use crate::state::*;
    pub use crate::types::*;
}
