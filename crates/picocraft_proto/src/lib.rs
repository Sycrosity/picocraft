#![no_std]
#![warn(
    clippy::absolute_paths,
    clippy::alloc_instead_of_core,
    clippy::disallowed_script_idents,
    clippy::doc_include_without_cfg,
    clippy::dbg_macro,
    clippy::else_if_without_else,
    clippy::empty_enum_variants_with_brackets,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::iter_over_hash_type,
    clippy::large_include_file,
    clippy::mod_module_files,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_raw_strings,
    clippy::panic_in_result_fn,
    clippy::precedence_bits,
    clippy::rc_mutex,
    clippy::redundant_test_prefix,
    clippy::renamed_function_params,
    clippy::same_name_method,
    clippy::string_lit_chars_any,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unused_result_ok,
    clippy::unwrap_used
)]

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
