#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod logger;

pub mod prelude {

    pub use log::{debug, error, info, log, trace, warn};
    pub use picocraft_core::prelude::*;
    pub use picocraft_server::prelude::*;
}
