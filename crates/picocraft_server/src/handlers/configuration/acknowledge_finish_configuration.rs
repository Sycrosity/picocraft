use picocraft_proto::chunks::*;
use picocraft_proto::serverbound::configuration::AcknowledgeFinishConfigurationPacket;

use crate::buffer::ByteCountWriter;
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

        let vec = PrefixedArray::from_vec(Vec::from_array([Identifier(
            String::try_from("overworld").expect("max 16 bytes"),
        )]));

        let login_play = clientbound::LoginPlayPacket::builder()
            .dimension_names(vec)
            .is_hardcore(false)
            .view_distance(VarInt(16))
            .simulation_distance(VarInt(16))
            .game_mode(1)
            .build();

        // trace!("Packet constructed: {:?}", login_play);

        client.encode_packet(&login_play).await?;

        let synchronise_player_position = clientbound::SynchronisePlayerPositionPacket::builder()
            .x(0f64)
            .z(0f64)
            .y(156f64)
            .build();

        // trace!("Packet constructed: {:?}", &synchronise_player_position);

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

        let mut data = Array::new();

        data.resize(
            8,
            ChunkSection {
                block_count: 4096,
                block_states: BlockPalettedContainer {
                    bits_per_entry: 0,
                    palette: Palette::SingleValued(VarInt(18)),
                    data: Array::new(),
                },
                biomes: BiomePalettedContainer {
                    bits_per_entry: 0,
                    palette: Palette::SingleValued(VarInt(0)),
                    data: Array::new(),
                },
            },
        )
        .unwrap();

        let empty_chunk = ChunkSection {
            block_count: 0,
            block_states: BlockPalettedContainer {
                bits_per_entry: 0,
                palette: Palette::SingleValued(VarInt(0)),
                data: Array::new(),
            },
            biomes: BiomePalettedContainer {
                bits_per_entry: 0,
                palette: Palette::SingleValued(VarInt(0)),
                data: Array::new(),
            },
        };

        data.resize(16, empty_chunk.clone()).unwrap();

        let mut counting_writer = ByteCountWriter::new();

        data.encode(&mut counting_writer).await?;

        let data_size = counting_writer.count;

        let mut light_arrays = PrefixedArray::new();

        light_arrays.resize(18, FullSkyLightSection).unwrap();

        let mut chunk_data_and_update_light = clientbound::ChunkDataAndUpdateLightPacket::builder()
            .chunk_x(0)
            .chunk_z(0)
            .data(ChunkData {
                heightmaps: PrefixedArray::new(),
                size: VarInt(data_size as i32),
                data: data.clone(),
                block_entities: PrefixedArray::new(),
            })
            .light(LightData {
                sky_light_mask: BitSet(
                    PrefixedArray::from_vec(Vec::from_array([0x3ffff]))
                ),
                block_light_mask: BitSet(
                    PrefixedArray::from_vec(Vec::from_array([0x3ffff]))
                ),
                empty_sky_light_mask: BitSet(PrefixedArray::new()),
                empty_block_light_mask: BitSet(PrefixedArray::new()),
                sky_light_arrays: light_arrays.clone(),
                block_light_arrays: light_arrays.clone(),
            })
            .build();

        for x in -10i32..10 {
            for z in -10i32..10 {
                if (2 * x + 1).abs() > 15 || (2 * z + 1).abs() > 15 {
                    chunk_data_and_update_light
                        .data
                        .data
                        .fill(empty_chunk.clone());
                } else {
                    chunk_data_and_update_light.data.data = data.clone();
                }

                chunk_data_and_update_light.chunk_x = x;
                chunk_data_and_update_light.chunk_z = z;

                for i in 0..8 {
                    chunk_data_and_update_light.data.data[i]
                        .block_states
                        .palette = Palette::SingleValued(VarInt(
                        1 + (2 * x + 1).abs().max((2 * z + 1).abs()) / 2,
                    ));
                }

                client.encode_packet(&chunk_data_and_update_light).await?;
            }
        }

        debug!(
            "Finished sending chunks to {} [{}]",
            client.player.username(),
            client.player.uuid()
        );

        Ok(())
    }
}
