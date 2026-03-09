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

pub mod component_store;
pub mod components;
pub mod entity;
pub mod pools;
pub mod world;

pub use world::World;

#[allow(unused)]
pub mod prelude {

    pub(crate) use heapless::Vec;
    pub(crate) use picocraft_core::prelude::*;
    // pub(crate) use picocraft_proto::prelude::*;
}
