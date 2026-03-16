use picocraft_proto::serverbound::play::ConfirmTeleportationPacket;

use crate::prelude::*;

impl HandlePacket for ConfirmTeleportationPacket {
    async fn handle(self, _client: &mut Client) -> Result<(), PacketError> {
        if *self.0 == 0 {
            Ok(())
        } else {
            warn!("Received invalid teleportation confirmation ID: {}", self.0);
            Err(PacketError::InvalidPacket(*Self::ID, Self::STATE))
        }
    }
}
