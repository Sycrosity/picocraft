use crate::prelude::*;

use picocraft_proto::serverbound::configuration::*;

impl HandlePacket for AcknowledgeFinishConfigurationPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        info!(
            "Client {} [{}] has finished configuration.",
            client.player.username(), client.player.uuid()
        );

        client.set_state(State::Play);

        Ok(())
    }
}