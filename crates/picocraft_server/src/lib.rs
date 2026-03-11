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

#[cfg(feature = "std")]
extern crate std;

pub mod channels;
pub mod client;
pub mod config;
pub mod errors;
pub mod handlers;
pub mod server;
pub mod shutdown;

pub use client::Client;
pub use server::Server;

#[allow(unused)]
pub mod prelude {

    pub(crate) use embedded_io_async::{Read, Write};
    pub(crate) use heapless::Vec;
    pub(crate) use log::{debug, error, info, log, trace, warn};
    pub(crate) use picocraft_core::prelude::*;
    pub(crate) use picocraft_proto::prelude::*;
    pub(crate) use rand::prelude::*;

    pub use crate::SystemRng;
    pub use crate::client::Client;
    pub use crate::config::ServerConfig;
    pub(crate) use crate::errors::*;
    pub(crate) use crate::handlers::HandlePacket;
    pub use crate::server::Server;
}

use core::cell::RefCell;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use rand_chacha::ChaCha8Rng;

pub type SystemRng = Mutex<CriticalSectionRawMutex, RefCell<ChaCha8Rng>>;
