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

pub mod buffer;
pub mod client;
pub mod config;
pub mod errors;
pub mod handlers;
pub mod packet_socket;
pub mod server;
pub mod shutdown;

pub use client::Client;
pub use server::Server;

#[allow(unused)]
pub mod prelude {

    pub(crate) use embedded_io_async::{Read, Write};
    pub(crate) use log::{debug, error, info, log, trace, warn};
    pub(crate) use picocraft_core::prelude::*;
    pub(crate) use picocraft_proto::prelude::*;
    pub(crate) use rand::prelude::*;

    pub(crate) use crate::buffer::Buffer;
    pub use crate::client::{Client, Player};
    pub(crate) use crate::errors::*;
    pub(crate) use crate::handlers::HandlePacket;
    pub use crate::server::Server;
    pub use crate::{ServerConfig, SystemRng};
}

pub type SystemRng = embassy_sync::mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    core::cell::RefCell<rand_chacha::ChaCha8Rng>,
>;

pub type ServerConfig = embassy_sync::rwlock::RwLock<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    crate::config::Config,
>;
