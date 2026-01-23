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

        let mut world = picocraft_terrain::world::World::new(0);

        world.generate_terrain_map();

        let mut data = Array::new();

        let empty_chunk = ChunkSectionProto {
            block_count: 0,
            block_states: BlockPalettedContainerProto {
                bits_per_entry: 0,
                palette: Palette::SingleValued(VarInt(0)),
                data: Array::new(),
            },
            biomes: BiomePalettedContainerProto {
                bits_per_entry: 0,
                palette: Palette::SingleValued(VarInt(0)),
                data: Array::new(),
            },
        };

        data.resize(16, empty_chunk.clone())
            .expect("Max array length should be 16");

        let mut counting_writer = ByteCountWriter::new();

        data.encode(&mut counting_writer).await?;

        let empty_data_size = counting_writer.count;

        let mut light_arrays = PrefixedArray::new();

        light_arrays
            .resize(18, FullSkyLightSection)
            .expect("Max array length should be 18");

        let mut chunk_data_and_update_light = clientbound::ChunkDataAndUpdateLightPacket::builder()
            .chunk_x(0)
            .chunk_z(0)
            .data(ChunkDataProto {
                heightmaps: PrefixedArray::new(),
                size: VarInt(empty_data_size as i32),
                data: data.clone(),
                block_entities: PrefixedArray::new(),
            })
            .light(LightDataProto {
                sky_light_mask: BitSet(PrefixedArray::from_vec(Vec::from_array([0x3ffff]))),
                block_light_mask: BitSet(PrefixedArray::from_vec(Vec::from_array([0x3ffff]))),
                empty_sky_light_mask: BitSet(PrefixedArray::new()),
                empty_block_light_mask: BitSet(PrefixedArray::new()),
                sky_light_arrays: light_arrays.clone(),
                block_light_arrays: light_arrays.clone(),
            })
            .build();

        for x in -9i32..9 {
            'outer: for z in -9i32..9 {
                if (2 * x + 1).abs() > 15 || (2 * z + 1).abs() > 15 {
                    chunk_data_and_update_light
                        .data
                        .data
                        .resize(16, empty_chunk.clone())
                        .expect("Max array length should be 16");

                    chunk_data_and_update_light.data.size = VarInt(empty_data_size as i32);

                    chunk_data_and_update_light
                        .data
                        .data
                        .fill(empty_chunk.clone());

                    chunk_data_and_update_light.chunk_x = x;
                    chunk_data_and_update_light.chunk_z = z;

                    client.encode_packet(&chunk_data_and_update_light).await?;

                    continue 'outer;
                } else {
                    chunk_data_and_update_light.data.data = data.clone();

                    chunk_data_and_update_light.chunk_x = x;
                    chunk_data_and_update_light.chunk_z = z;
                }

                for y in 0..16 {
                    let mut palette = PrefixedArray::new();
                    let _ = palette.push(VarInt(0));
                    let _ = palette.push(VarInt(1));

                    chunk_data_and_update_light.data.data[y].block_count = 4096;
                    chunk_data_and_update_light.data.data[y]
                        .block_states
                        .palette = Palette::Indirect(palette);

                    chunk_data_and_update_light.data.data[y]
                        .block_states
                        .data
                        .resize(512, 0)
                        .expect("Max len should be 512");

                    // log::info!("{:?}",
                    // &chunk_data_and_update_light.data.data[chunk_index].block_states.data);

                    chunk_data_and_update_light.data.data[y]
                        .block_states
                        .bits_per_entry = 8;
                }

                for local_z in 0..16 {
                    for local_x in 0..16 {
                        let world_x = (x << 4) | local_x;
                        let world_z = (z << 4) | local_z;

                        // info!("{} {world_z}", world_x+128);
                        let hx = (world_x + 128) as usize;
                        let hz = (world_z + 128) as usize;
                        // info!("{hx} {hz}");
                        let h = world
                            .terrain_map
                            .get(hx, hz)
                            .expect("index should be in range")
                            as usize;

                        for world_y in 0..h {
                            let chunk_y = world_y >> 4;
                            let local_y = world_y & 15;

                            // world[chunk_y as usize]
                            //     .set_block(local_x as usize,
                            //             local_y as usize,
                            //             local_z as usize,
                            //             1);

                            // (y * 16 + z) * 16 + x
                            let idx = (local_y << 8) | ((local_z as usize) << 4) | local_x as usize;

                            let word = idx >> 3;
                            let byte = idx & 7;
                            let shift = byte << 3;

                            let dat = &mut chunk_data_and_update_light.data.data[chunk_y]
                                .block_states
                                .data;

                            let mask = !(0xffi64 << shift);
                            dat[word] = (dat[word] & mask) | ((1i64) << shift);
                        }
                        // log::info!("{:X?}",
                        // &chunk_data_and_update_light.data.data[0].
                        // block_states.data);
                        // log::info!("{:X?}",
                        // &chunk_data_and_update_light.data.data[12].
                        // block_states.data);
                    }
                }

                let mut counting_writer = ByteCountWriter::new();

                chunk_data_and_update_light
                    .data
                    .data
                    .encode(&mut counting_writer)
                    .await?;

                let data_size = counting_writer.count;

                chunk_data_and_update_light.data.size = VarInt(data_size as i32);

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
