use embassy_sync::pubsub::WaitResult;
use picocraft_ecs::commands::WorldCommand;
use picocraft_ecs::events::WorldEvent;
use picocraft_proto::serverbound::configuration::AcknowledgeFinishConfigurationPacket;
use picocraft_terrain::terrain::chunks::empty_chunk::EmptyChunkAndLightPacket;
use picocraft_terrain::terrain::coordinates::ChunkColumnCoordinates;
use picocraft_terrain::terrain::spiral_iterator::{BorderedSpiralIterator, ChunkKind};

use crate::channels::COMMANDS;
use crate::prelude::*;

impl HandlePacket for AcknowledgeFinishConfigurationPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        info!(
            "Client {} [{}] has finished configuration.",
            client.username().clone(),
            client.uuid()
        );

        client.set_state(State::Play);

        COMMANDS
            .send(WorldCommand::PlayerJoined {
                username: client.username().clone(),
                uuid: client.uuid(),
            })
            .await;

        let mut opt_position = None;
        let mut opt_rotation = None;

        let mut buffered_events: heapless::Vec<WorldEvent, MAX_PLAYERS> = heapless::Vec::new();

        let (entity_id, position, rotation) = loop {
            match client
                .events
                .as_mut()
                .expect("client should have a valid Subscriber by this point")
                .next_message()
                .await
            {
                WaitResult::Message(WorldEvent::PlayerJoined {
                    player_id,
                    uuid,
                    position,
                    rotation,
                    ..
                }) if uuid == client.uuid() => {
                    client.entity_id = Some(player_id);
                    opt_position.replace(position);
                    opt_rotation.replace(rotation);
                }
                WaitResult::Message(WorldEvent::WorldReady { recipient })
                    if Some(recipient) == client.entity_id =>
                {
                    break (
                        recipient,
                        opt_position.expect("we set this"),
                        opt_rotation.expect("we set this"),
                    );
                }
                WaitResult::Message(event @ WorldEvent::ExistingPlayer { .. }) => {
                    // ignore other messages until we get the PlayerJoined event and any
                    // ExistingPlayer events for this client since we need to
                    // know our entity id before doing anything else.

                    let _ = buffered_events.push(event);
                }
                WaitResult::Message(_) => {}
                WaitResult::Lagged(skipped) => {
                    // in theory this should be unreachable or very close to impossible
                    warn!(
                        "Lagged while waiting for PlayerJoined event, skipped {skipped} messages."
                    );
                }
            }
        };

        let vec: PrefixedArray<Identifier<16>, 3> = PrefixedArray::from_array([Identifier(
            String::try_from("overworld").expect("max 16 bytes"),
        )]);

        let login_play = clientbound::LoginPlayPacket::builder()
            .entity_id(i32::from(entity_id.index()))
            .dimension_names(vec)
            .is_hardcore(false)
            .view_distance(VarInt(16))
            .simulation_distance(VarInt(16))
            .game_mode(1)
            .build();

        client.encode_packet(&login_play).await?;

        let synchronise_player_position = clientbound::SynchronisePlayerPositionPacket::builder()
            .x(position.protocol_x())
            .z(position.protocol_z())
            .y(position.protocol_y())
            .pitch(rotation.pitch)
            .yaw(rotation.yaw)
            .build();

        client.encode_packet(&synchronise_player_position).await?;

        let actions = EnumSet::ADD_PLAYER | EnumSet::UPDATE_LISTED | EnumSet::UPDATE_GAME_MODE;

        let mut players = PrefixedArray::new();

        let mut action_array = Array::new();

        let _ = action_array.push(clientbound::PlayerActions::AddPlayer {
            username: client.username().clone(),
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

        let spawn = ChunkColumnCoordinates::new(0, 0);

        for (x, z, kind) in BorderedSpiralIterator::new(16, spawn) {
            match kind {
                ChunkKind::Terrain => {
                    let chunk = client.terrain.get_chunk_packet(x, z);
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

        for event in buffered_events {
            client.handle_event(event).await?;
        }

        Ok(())
    }
}
