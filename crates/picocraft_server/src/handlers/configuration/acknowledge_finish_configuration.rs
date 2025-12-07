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

        client.encode_packet(&login_play).await?;

        let synchronise_player_position = clientbound::SynchronisePlayerPositionPacket::builder()
            .x(0f64)
            .z(0f64)
            .y(156f64)
            .build();

        trace!("Packet constructed: {:?}", &synchronise_player_position);

        client.encode_packet(&synchronise_player_position).await?;

        let actions = EnumSet::new().add_player().update_listed();

        let mut players = PrefixedArray::new();

        let mut action_array = Array::new();

        let _ = action_array.push(clientbound::PlayerActions::AddPlayer {
            name: client.username().clone(),
            properties: Properties::default(),
        });

        let _ = action_array.push(clientbound::PlayerActions::UpdateListed(true));

        let _ = players.push((client.uuid(), action_array));

        let player_info_update = clientbound::PlayerInfoUpdatePacket::<2> { actions, players };

        trace!("Packet constructed: {:?}", &player_info_update);

        client.encode_packet(&player_info_update).await?;

        let game_event = clientbound::GameEventPacket::builder()
            .event(clientbound::GameEvent::StartWaitingForLevelChunks)
            .build();

        trace!("Packet constructed: {:?}", &game_event);

        client.encode_packet(&game_event).await?;


        Ok(())
    }
}
