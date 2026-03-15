mod confirm_teleportation;
mod player;

use picocraft_proto::serverbound::{ClientTickEndPacket, ServerboundKeepAlivePacket};

use crate::prelude::*;
impl HandlePacket for ClientTickEndPacket {
    async fn handle(self, _client: &mut Client) -> Result<(), PacketError> {
        Ok(())
    }
}

impl HandlePacket for ServerboundKeepAlivePacket {
    async fn handle(self, _client: &mut Client) -> Result<(), PacketError> {
        Ok(())
    }
}
