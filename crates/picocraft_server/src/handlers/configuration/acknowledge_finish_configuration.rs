use picocraft_proto::serverbound::configuration::*;

use crate::prelude::*;

impl HandlePacket for AcknowledgeFinishConfigurationPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        info!(
            "Client {} [{}] has finished configuration.",
            client.player.username(),
            client.player.uuid()
        );

        client.set_state(State::Play);

        let vec = PrefixedArray::from_slice(&[Identifier(
            String::try_from("overworld").expect("max 16 bytes"),
        )])
        .expect("max 3 dimensions");

        let login_play = clientbound::LoginPlayPacket::builder()
            .dimension_names(vec)
            .is_hardcore(false)
            .view_distance(VarInt(8))
            .simulation_distance(VarInt(8))
            .build();

        trace!("Packet constructed: {:?}", login_play);

        login_play.encode(&mut client.tx_buf).await?;

        client.encode_packet_length(client.tx_buf.len()).await?;
        client.socket.write_all(&client.tx_buf).await?;
        client.socket.flush().await?;
        client.tx_buf.clear();

        trace!("Login (Play) packet sent.");

        Ok(())
    }
}
