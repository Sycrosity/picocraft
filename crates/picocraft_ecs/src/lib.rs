#![cfg_attr(not(test), no_std)]
#![deny(clippy::mem_forget)]
#![warn(
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::large_include_file,
    clippy::panic_in_result_fn,
    clippy::std_instead_of_core,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::undocumented_unsafe_blocks,
    clippy::unwrap_used
)]

pub mod component;
pub mod entity;
pub mod sparse_set;
pub mod system;
pub mod world;

pub mod prelude {
    pub use crate::component::*;
    pub use crate::entity::*;
    pub use crate::sparse_set::SparseSet;
    pub use crate::system::System;
    pub use crate::world::EcsWorld;
}
