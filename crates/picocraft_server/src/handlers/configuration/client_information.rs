use picocraft_proto::serverbound::configuration::ClientInformationPacket;

use crate::prelude::*;

impl HandlePacket for ClientInformationPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        client.player.set_client_info(self.0);

        Ok(())
    }
}
