use picocraft_proto::serverbound::configuration::AcknowledgeFinishConfigurationPacket;
use picocraft_terrain::world::chunks::empty_chunk::EmptyChunkAndLightPacket;
use picocraft_terrain::world::coordinates::ChunkColumnCoordinates;
use picocraft_terrain::world::spiral_iterator::ChunkKind;

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

        let vec: PrefixedArray<Identifier<16>, 3> = PrefixedArray::from_array([Identifier(
            String::try_from("overworld").expect("max 16 bytes"),
        )]);

        let login_play = clientbound::LoginPlayPacket::builder()
            .dimension_names(vec)
            .is_hardcore(false)
            .view_distance(VarInt(16))
            .simulation_distance(VarInt(16))
            .game_mode(1)
            .build();

        client.encode_packet(&login_play).await?;

        let synchronise_player_position = clientbound::SynchronisePlayerPositionPacket::builder()
            .x(0f64)
            .z(0f64)
            .y(156f64)
            .build();

        client.encode_packet(&synchronise_player_position).await?;

        let actions = EnumSet::new()
            .add_player()
            .update_listed()
            .update_game_mode();

        let mut players = PrefixedArray::new();

        let mut action_array = Array::new();

        let _ = action_array.push(clientbound::PlayerActions::AddPlayer {
            name: client.username().clone(),
            properties: Properties::default(),
        });

        let _ = action_array.push(clientbound::PlayerActions::UpdateListed(true));

        let _ = action_array.push(clientbound::PlayerActions::UpdateGameMode(VarInt(1)));

        let _ = players.push((client.uuid(), action_array));

        let player_info_update = clientbound::PlayerInfoUpdatePacket::<3> { actions, players };

        trace!("Packet constructed: {:?}", &player_info_update);

        client.encode_packet(&player_info_update).await?;

        let initialise_world_border = clientbound::InitialiseWorldBorderPacket::default();

        trace!("Packet constructed: {:?}", &initialise_world_border);

        client.encode_packet(&initialise_world_border).await?;

        let game_event = clientbound::GameEventPacket::builder()
            .event(clientbound::GameEvent::StartWaitingForLevelChunks)
            .build();

        trace!("Packet constructed: {:?}", &game_event);

        client.encode_packet(&game_event).await?;

        let mut world = picocraft_terrain::world::World::new(0);
        world.generate_terrain_map();

        let spawn = ChunkColumnCoordinates::new(0, 0);

        for (x, z, kind) in
            picocraft_terrain::world::spiral_iterator::BorderedSpiralIterator::new(8, spawn)
        {
            match kind {
                ChunkKind::Terrain => {
                    let chunk = world.get_chunk_packet(x, z);
                    client.encode_packet(&chunk).await?;
                }
                ChunkKind::Air => {
                    let empty = EmptyChunkAndLightPacket::new(x, z);
                    client.encode_packet(&empty).await?;
                }
            }
        }

        trace!("Packets sent: ChunkAndLightPacket");

        debug!(
            "Finished sending chunks to {} [{}]",
            client.player.username(),
            client.player.uuid()
        );

        Ok(())
    }
}
