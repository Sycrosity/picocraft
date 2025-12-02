use picocraft_proto::serverbound::login::*;

use crate::prelude::*;

impl HandlePacket for LoginStartPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        client.player.set_username(self.username);
        client.player.set_uuid(self.uuid);

        let login_success = clientbound::LoginSuccess(GameProfile::new(
            client.player.username().clone(),
            client.player.uuid(),
        ));

        login_success
            .encode(&mut client.tx_buf)
            .await
            .inspect_err(|e| error!("{e:#?}"))?;

        trace!("Packet constructed: {:?}", login_success);

        client.encode_packet_length(client.tx_buf.len()).await?;

        client.socket.write_all(&client.tx_buf).await?;

        client.socket.flush().await?;

        trace!("Login Success packet sent.");

        Ok(())
    }
}

impl HandlePacket for LoginAcknowledgedPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        debug!(
            "{} [{}] has logged in.",
            &client.username(),
            &client.uuid()
        );

        client.set_state(State::Configuration);

        Ok(())
    }
}
