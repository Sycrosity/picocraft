use picocraft_proto::serverbound::play::LoginStartPacket;

use crate::prelude::*;

impl HandlePacket for LoginStartPacket {
    async fn handle(self, _client: &mut Client) -> Result<(), PacketError> {
        todo!("Handle LoginStartPacket is not yet implemented")
    }
}
