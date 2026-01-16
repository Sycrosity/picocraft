use eden_growth::biomes::Biome;
use eden_growth::config::{RIVER_TILE_SIZE, TILE_SIZE};
use eden_growth::eden::generate_eden_tile_grid;
use eden_growth::river::overlay_rivers;
use picocraft_proto::chunks::*;
use picocraft_proto::serverbound::configuration::AcknowledgeFinishConfigurationPacket;

use crate::buffer::ByteCountWriter;
use crate::prelude::*;

// Placeholder block IDs — replace with real block state IDs from picocraft_core.
const BLOCK_ID_OCEAN: VarInt = VarInt(101);
const BLOCK_ID_PLAINS: VarInt = VarInt(9);
const BLOCK_ID_MOUNTAINS: VarInt = VarInt(1);
const BLOCK_ID_DESERT: VarInt = VarInt(118);
const BLOCK_ID_TAIGA: VarInt = VarInt(13);
const BLOCK_ID_SAVANNA: VarInt = VarInt(10);
const BLOCK_ID_FOREST: VarInt = VarInt(137);
const BLOCK_ID_RIVER: VarInt = VarInt(565);

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
            .view_distance(VarInt(16))
            .simulation_distance(VarInt(16))
            .game_mode(1)
            .build();

        trace!("Packet constructed: {:?}", login_play);

        client.encode_packet(&login_play).await?;

        let synchronise_player_position = clientbound::SynchronisePlayerPositionPacket::builder()
            .x(64f64)
            .z(64f64)
            .y(64f64)
            .build();

        trace!("Packet constructed: {:?}", &synchronise_player_position);

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

        let game_event = clientbound::GameEventPacket::builder()
            .event(clientbound::GameEvent::StartWaitingForLevelChunks)
            .build();

        trace!("Packet constructed: {:?}", &game_event);

        client.encode_packet(&game_event).await?;

        // === Eden growth chunk streaming ===
        // Generate a single Eden tile (64x64 biomes => 128x128 blocks) and stream its 8x8 chunk grid,
        // surrounded by a 1-chunk air border so the client accepts the terrain.
        const BLOCKS_PER_BIOME: usize = 2; // 2x2 blocks per biome cell
        const CHUNK_SIZE: usize = 16;
        const TILE_BLOCKS: usize = TILE_SIZE * BLOCKS_PER_BIOME; // 128
        const CHUNKS_PER_AXIS: usize = TILE_BLOCKS / CHUNK_SIZE; // 8
        const BORDER: i32 = 1; // one chunk of air on each side

        let eden_seed: u64 = 123_456_789; // TODO: choose a seed source (per world/session)

        let grid_opt = generate_eden_tile_grid(eden_seed);
        let (grid_opt, river_cells, _knots) = overlay_rivers(grid_opt, eden_seed);
        let grid: [[Biome; TILE_SIZE]; TILE_SIZE] =
            grid_opt.map(|row| row.map(|b| b.unwrap_or(Biome::Plains)));
        let river_mask = build_river_mask(river_cells.as_slice());

        let empty_section = ChunkSection {
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

        let mut light_arrays = PrefixedArray::new();
        light_arrays.resize(18, FullSkyLightSection).unwrap();

        for chunk_z in -(BORDER)..(CHUNKS_PER_AXIS as i32 + BORDER) {
            for chunk_x in -(BORDER)..(CHUNKS_PER_AXIS as i32 + BORDER) {
                // Border chunks are all air.
                let sections: Array<ChunkSection<256>, 16> = if chunk_x < 0
                    || chunk_z < 0
                    || chunk_x >= CHUNKS_PER_AXIS as i32
                    || chunk_z >= CHUNKS_PER_AXIS as i32
                {
                    let mut a = Array::new();
                    a.resize(16, empty_section.clone()).unwrap();
                    a
                } else {
                    let section = build_chunk_section(
                        &grid,
                        &river_mask,
                        chunk_x as usize,
                        chunk_z as usize,
                        BLOCKS_PER_BIOME,
                    );
                    let mut a = Array::new();
                    a.resize(16, empty_section.clone()).unwrap();
                    if let Some(slot) = a.get_mut(0) {
                        *slot = section;
                    }
                    a
                };

                // Compute encoded size for the chunk data field.
                let mut counting_writer = ByteCountWriter::new();
                sections.encode(&mut counting_writer).await?;
                let data_size = counting_writer.count;

                let chunk_data_and_update_light = clientbound::ChunkDataAndUpdateLightPacket::builder()
                    .chunk_x(chunk_x)
                    .chunk_z(chunk_z)
                    .data(ChunkData {
                        heightmaps: PrefixedArray::new(),
                        size: VarInt(data_size as i32),
                        data: sections,
                        block_entities: PrefixedArray::new(),
                    })
                    .light(LightData {
                        sky_light_mask: BitSet(PrefixedArray::from_slice(&[0x3ffff]).unwrap()),
                        block_light_mask: BitSet(PrefixedArray::from_slice(&[0x3ffff]).unwrap()),
                        empty_sky_light_mask: BitSet(PrefixedArray::new()),
                        empty_block_light_mask: BitSet(PrefixedArray::new()),
                        sky_light_arrays: light_arrays.clone(),
                        block_light_arrays: light_arrays.clone(),
                    })
                    .build();

                info!("chunk_x: {chunk_x} & chunk_z: {chunk_z}");

                client.encode_packet(&chunk_data_and_update_light).await?;
            }
        }

        Ok(())
    }
}

fn block_id_for_biome(b: Biome) -> VarInt {
    match b {
        Biome::Ocean => BLOCK_ID_OCEAN,
        Biome::Plains => BLOCK_ID_PLAINS,
        Biome::Mountains => BLOCK_ID_MOUNTAINS,
        Biome::Desert => BLOCK_ID_DESERT,
        Biome::Taiga => BLOCK_ID_TAIGA,
        Biome::Savanna => BLOCK_ID_SAVANNA,
        Biome::Forest => BLOCK_ID_FOREST,
        Biome::River => BLOCK_ID_RIVER,
    }
}

fn build_river_mask(river: &[(u16, u16)]) -> [[bool; RIVER_TILE_SIZE]; RIVER_TILE_SIZE] {
    let mut mask = [[false; RIVER_TILE_SIZE]; RIVER_TILE_SIZE];
    for &(x, y) in river.iter() {
        let ux = x as usize;
        let uy = y as usize;
        if ux < RIVER_TILE_SIZE && uy < RIVER_TILE_SIZE {
            mask[uy][ux] = true;
        }
    }
    mask
}

fn build_chunk_section(
    grid: &[[Biome; TILE_SIZE]; TILE_SIZE],
    river_mask: &[[bool; RIVER_TILE_SIZE]; RIVER_TILE_SIZE],
    chunk_x: usize,
    chunk_z: usize,
    blocks_per_biome: usize,
) -> ChunkSection<256> {
    const CHUNK_SIZE: usize = 16;

    let mut palette: PrefixedArray<VarInt, 256> = PrefixedArray::new();
    let mut indices: Vec<u16, { CHUNK_SIZE * CHUNK_SIZE }> = Vec::new();

    let mut add_palette = |id: VarInt, palette: &mut PrefixedArray<VarInt, 256>| -> u16 {
        if let Some((idx, _)) = palette.iter().enumerate().find(|(_, v)| **v == id) {
            idx as u16
        } else {
            let idx = palette.len() as u16;
            let _ = palette.push(id);
            idx
        }
    };

    for local_z in 0..CHUNK_SIZE {
        for local_x in 0..CHUNK_SIZE {
            let global_x = chunk_x * CHUNK_SIZE + local_x;
            let global_z = chunk_z * CHUNK_SIZE + local_z;

            let river_hit = river_mask
                .get(global_z)
                .and_then(|row| row.get(global_x))
                .copied()
                .unwrap_or(false);

            let biome = if river_hit {
                Biome::River
            } else {
                let bx = global_x / blocks_per_biome;
                let bz = global_z / blocks_per_biome;
                grid[bz][bx]
            };

            let block_id = block_id_for_biome(biome);
            let palette_idx = add_palette(block_id, &mut palette);
            let _ = indices.push(palette_idx);
        }
    }

    // Repeat the 16x16 layer across 16 vertical blocks.
    let mut full_indices: Vec<u16, { CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE }> = Vec::new();
    for _y in 0..CHUNK_SIZE {
        let _ = full_indices.extend_from_slice(&indices);
    }

    // If only one block ID is present, use SingleValued with bits_per_entry = 0.
    if palette.len() == 1 {
        ChunkSection {
            block_count: 4096,
            block_states: BlockPalettedContainer {
                bits_per_entry: 0,
                palette: Palette::SingleValued(palette[0]),
                data: Array::new(),
            },
            biomes: BiomePalettedContainer {
                bits_per_entry: 0,
                palette: Palette::SingleValued(VarInt(0)),
                data: Array::new(),
            },
        }
    } else {
        let bits_per_entry = compute_bits_per_entry(palette.len() as u16);
        let data = pack_indices(&full_indices, bits_per_entry);

        ChunkSection {
            block_count: 4096,
            block_states: BlockPalettedContainer {
                bits_per_entry: bits_per_entry as u8,
                palette: Palette::Indirect(palette),
                data,
            },
            biomes: BiomePalettedContainer {
                bits_per_entry: 0,
                palette: Palette::SingleValued(VarInt(0)),
                data: Array::new(),
            },
        }
    }
}

fn compute_bits_per_entry(palette_len: u16) -> u32 {
    if palette_len == 0 {
        4
    } else {
        let raw = 16u32.saturating_sub((palette_len - 1).leading_zeros());
        raw.max(4).min(8)
    }
}

fn pack_indices<const N: usize>(indices: &Vec<u16, N>, bits_per_entry: u32) -> Array<Long, 1024> {
    const BLOCK_DATA_CAP: usize = 1024;

    let mut data: Array<Long, { BLOCK_DATA_CAP }> = Array::new();
    if bits_per_entry == 0 {
        return data;
    }

    let entries_per_long = 64 / bits_per_entry;
    let required_longs = (4096 + entries_per_long - 1) / entries_per_long;
    let mask = (1u64 << bits_per_entry) - 1;

    let mut iter = indices.iter().take(4096).copied();
    for _ in 0..required_longs {
        let mut acc: u64 = 0;
        for i in 0..entries_per_long {
            if let Some(idx) = iter.next() {
                acc |= (idx as u64 & mask) << (i * bits_per_entry);
            } else {
                break; // remaining bits in this long stay as padding
            }
        }
        let _ = data.push(acc as i64);
    }

    data
}
