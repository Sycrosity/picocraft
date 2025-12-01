use picocraft_proto::serverbound::handshake::*;

use crate::prelude::*;

impl HandlePacket for HandshakePacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        client.player.set_protocol_version(self.protocol_version);

        client.set_state(match self.intent {
            Intent::Status => State::Status,
            Intent::Login => State::Login,
            Intent::Transfer => {
                panic!("Transfer intent not supported yet");
            }
        });

        Ok(())
    }
}

impl HandlePacket for LegacyPingPacket {
    async fn handle(self, _client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        todo!("can't handle legacy ping packets yet")
    }
}
