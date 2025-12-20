use crate::prelude::*;

#[derive(Debug, Clone, Packet)]
#[packet(id = 0x2b)]
pub struct KeepAlivePacket {
    keep_alive_id: Long,
}

impl Default for KeepAlivePacket {
    fn default() -> Self {
        Self::new(0)
    }
}

impl KeepAlivePacket {
    #[must_use]
    pub fn new(keep_alive_id: Long) -> Self {
        Self { keep_alive_id }
    }

    pub fn id(&self) -> Long {
        self.keep_alive_id
    }
}
