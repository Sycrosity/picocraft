use picocraft_proto::serverbound::configuration::AcknowledgeFinishConfigurationPacket;

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

        clientbound::SynchronisePlayerPositionPacket::default()
            .encode(&mut client.tx_buf)
            .await?;

        client.encode_packet_length(client.tx_buf.len()).await?;
        client.socket.write_all(&client.tx_buf).await?;
        client.socket.flush().await?;
        client.tx_buf.clear();

        trace!("Synchronise Player Position packet sent.");

        let actions = EnumSet::new().add_player().update_listed();

        let mut players = PrefixedArray::new();

        let mut action_array = Array::new();

        let _ = action_array.push(clientbound::PlayerActions::AddPlayer {
            name: client.username().clone(),
            properties: Properties::default(),
        });

        let _ = action_array.push(clientbound::PlayerActions::UpdateListed(true));

        let _ = players.push((client.uuid(), action_array));

        let player_info_update = clientbound::PlayerInfoUpdatePacket::<1, 2> { actions, players };

        trace!("Packet constructed: {:?}", &player_info_update);

        player_info_update.encode(&mut client.tx_buf).await?;

        client.encode_packet_length(client.tx_buf.len()).await?;
        client.socket.write_all(&client.tx_buf).await?;
        client.socket.flush().await?;
        client.tx_buf.clear();

        trace!("Player Info Update packet sent.");

        let game_event = clientbound::GameEventPacket::builder()
            .event(clientbound::GameEvent::StartWaitingForLevelChunks)
            .build();

        trace!("Packet constructed: {:?}", &game_event);

        game_event.encode(&mut client.tx_buf).await?;

        client.encode_packet_length(client.tx_buf.len()).await?;
        client.socket.write_all(&client.tx_buf).await?;
        client.socket.flush().await?;
        client.tx_buf.clear();

        trace!("Game Event packet sent.");

        let set_center_chunk = clientbound::SetCenterChunkPacket::default();

        trace!("Packet constructed: {:?}", &set_center_chunk);

        set_center_chunk.encode(&mut client.tx_buf).await?;

        client.encode_packet_length(client.tx_buf.len()).await?;
        client.socket.write_all(&client.tx_buf).await?;
        client.socket.flush().await?;
        client.tx_buf.clear();

        Ok(())
    }
}
