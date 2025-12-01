#![no_std]
#![allow(
    clippy::needless_continue,
    clippy::single_match_else,
    clippy::wildcard_imports
)]
//These should maybe be fixed properly later
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
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

pub use server::Server;

#[allow(unused)]
pub mod prelude {

    pub(crate) use embedded_io_async::{Read, Write};
    pub(crate) use log::{debug, error, info, log, trace, warn};
    pub(crate) use picocraft_core::prelude::*;
    pub(crate) use picocraft_proto::prelude::*;

    pub(crate) use crate::buffer::Buffer;
    pub use crate::client::Client;
    pub(crate) use crate::errors::*;
    pub(crate) use crate::handlers::HandlePacket;
    pub(crate) use crate::server::SERVER_CONFIG;
    pub use crate::server::Server;
}
