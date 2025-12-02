#![no_std]
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

pub mod logger;

pub mod prelude {

    pub use log::{debug, error, info, log, trace, warn};
    pub use picocraft_core::prelude::*;
    pub use picocraft_server::prelude::*;
}
