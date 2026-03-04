use picocraft_proto::serverbound::play::ClientTickEndPacket;

use crate::prelude::*;

impl HandlePacket for ClientTickEndPacket {
    async fn handle(self, _client: &mut Client) -> Result<(), PacketError> {
        // trace!("Packet received: {:?}", &self);

        Ok(())
    }
}
