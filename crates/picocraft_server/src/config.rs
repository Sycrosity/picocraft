use core::net::Ipv4Addr;

use crate::prelude::*;

#[derive(Debug)]
pub struct Config {
    pub address: Ipv4Addr,
    pub port: u16,
    pub motd: String<128>,
    pub max_players: i32,
}

impl Config {
    pub const fn default() -> Self {
        Self {
            address: Ipv4Addr::UNSPECIFIED,
            port: 25565,
            motd: String::new(),
            max_players: 8,
        }
    }
}
