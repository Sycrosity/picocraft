use crate::prelude::*;

#[derive(Debug, Clone, Packet)]
#[packet(id = 0x2b)]
pub struct KeepAlivePacket {
    keep_alive_id: Long,
}

impl Default for KeepAlivePacket {
    fn default() -> Self {
        Self::new()
    }
}

impl KeepAlivePacket {
    #[must_use]
    pub fn new() -> Self {
        Self {
            keep_alive_id: /*embassy_time::Instant::now().as_millis() as i64*/ 0,
        }
    }

    pub fn id(&self) -> Long {
        self.keep_alive_id
    }
}
