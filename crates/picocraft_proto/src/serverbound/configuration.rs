mod client_information;

pub use client_information::*;

use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x03, state = State::Configuration)]
pub struct AcknowledgeFinishConfigurationPacket;
