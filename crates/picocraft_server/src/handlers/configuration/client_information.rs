use picocraft_proto::serverbound::configuration::*;

use crate::prelude::*;

impl HandlePacket for ClientInformationPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        client.player.set_client_info(
            ClientInformation::builder()
                .locale(self.locale.clone())
                .view_distance(self.view_distance)
                .chat_mode(self.chat_mode)
                .chat_colors(self.chat_colors)
                .displayed_skin_parts(self.displayed_skin_parts)
                .enable_text_filtering(self.enable_text_filtering)
                .allow_server_listings(self.allow_server_listings)
                .particle_status(self.particle_status)
                .main_hand(self.main_hand)
                .build(),
        );

        clientbound::FinishConfigurationPacket
            .encode(&mut client.tx_buf)
            .await
            .inspect_err(|e| error!("{e:#?}"))?;

        client.encode_packet_length(client.tx_buf.len()).await?;

        client.socket.write_all(&client.tx_buf).await?;

        client.socket.flush().await?;

        trace!("Finish Configuration sent.");

        Ok(())
    }
}
