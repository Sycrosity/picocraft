use core::net::Ipv4Addr;
use core::str::FromStr;

use crate::prelude::*;

#[derive(Debug)]
pub struct ServerConfig {
    pub seed: u64,
    pub address: Ipv4Addr,
    pub port: u16,
    pub motd: String<128>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            address: Ipv4Addr::UNSPECIFIED,
            port: 25565,
            motd: heapless::String::from_str("A Picocraft Server!")
                .expect("String is less than 256 bytes"),
        }
    }
}
