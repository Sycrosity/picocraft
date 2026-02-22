pub mod biomes;
pub mod blocks;
pub mod chunks;
pub mod coordinates;
pub mod heightmaps;
pub mod light;
pub mod palettes;
pub mod spiral_iterator;

use blocks::IndexedBlock;
use coordinates::*;

use crate::noise::*;
use crate::prelude::*;
use crate::world::chunks::ChunkSection;
use crate::world::heightmaps::ChunkHeightmaps;

#[non_exhaustive]
pub struct World {
    seed: u64,
    terrain_map: NoiseMap256,
    /// The player y level considered to be "sea level", for which air blocks
    /// below this level are filled with water, before caves are applied.
    sea_level: u8,
}

impl World {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            terrain_map: NoiseMap256::new(),
            sea_level: 62,
        }
    }

    pub fn sea_level(&self) -> u8 {
        self.sea_level
    }

    pub fn set_sea_level(&mut self, sea_level: u8) {
        self.sea_level = sea_level;
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn get_chunk_packet(&mut self, chunk_x: i8, chunk_z: i8) -> chunks::ChunkAndLightPacket {
        chunks::ChunkAndLightPacket {
            chunk_x: Int::from(chunk_x),
            chunk_z: Int::from(chunk_z),
            chunk_data: self.get_chunk(chunk_x, chunk_z),
            light_data: light::LightData,
        }
    }

    pub fn get_chunk(&mut self, chunk_x: i8, chunk_z: i8) -> chunks::ChunkData {
        let mut heightmap = ChunkHeightmaps::new();

        let chunk_sections = (0..16)
            .map(|chunk_y| {
                self.get_chunk_section(
                    ChunkCoordinates::new(chunk_x, chunk_y, chunk_z),
                    &mut heightmap,
                )
            })
            .collect::<Vec<ChunkSection, 16>>();

        chunks::ChunkData {
            heightmaps: heightmap,
            data: Array::from_vec(chunk_sections),
            block_entities: PrefixedArray::new(),
        }
    }

    #[allow(unused_parens)]
    pub fn get_chunk_section(
        &mut self,
        chunk_coords: ChunkCoordinates,
        heightmap: &mut ChunkHeightmaps,
    ) -> chunks::ChunkSection {
        let mut packed_blocks: Array<u64, 256> = Array::new();

        let mut block_count: Short = 0;

        let bounds = chunk_coords.to_bounds();

        let mut accumulator: u64 = 0;
        let mut shift: u32 = 0;

        for coords in bounds.iter() {
            let indexed_block = self.get_indexed_block_at(coords.x, coords.y, coords.z);

            let local_x = coords.x.rem_euclid(16) as u8;
            let local_z = coords.z.rem_euclid(16) as u8;

            if indexed_block != IndexedBlock::Air {
                heightmap
                    .world_surface
                    .set(local_x, local_z, Some(coords.y));

                block_count += 1;
            }

            let value = indexed_block as u64;

            accumulator |= (value << shift);
            shift += 4;

            if shift >= 64 {
                packed_blocks
                    .push(accumulator)
                    .expect("should have space for 256 Longs in chunk section");
                accumulator = 0;
                shift = 0;
            }
        }

        chunks::ChunkSection {
            block_count,
            blocks: chunks::BlockContainer {
                bits_per_entry: 4,
                palette: self.get_palette(chunk_coords),
                packed_blocks,
            },
            biomes: chunks::BiomeContainer::default(),
        }
    }

    pub fn generate_terrain_map(&mut self) {
        // let mut random =
        // rand_xoshiro::Xoroshiro128PlusPlus::seed_from_u64(self.seed);

        let perlin: FbmPerlin = FbmPerlin::new(self.seed() as u32).set_octaves(4);

        //TODO this function is garbage.
        self.terrain_map.apply(|x, y| {
            (perlin.get([(x as f64 / 128.0 - 128.0), (y as f64 / 128.0 - 128.0)]) * 32.0 + 96.0)
                as u8
        });
    }

    #[inline]
    pub fn get_indexed_block_at(&self, x: i16, y: u8, z: i16) -> IndexedBlock {
        let height = self
            .terrain_map
            .get(x + 128, z + 128)
            .expect("index should be in range");

        if y > height {
            if y <= self.sea_level {
                IndexedBlock::Liquid
            } else {
                IndexedBlock::Air
            }
        } else if y == height {
            IndexedBlock::SurfaceBlock
        } else if y >= height.saturating_sub(4) {
            IndexedBlock::SubSurfaceBlock
        } else if y == 0 {
            IndexedBlock::Special
        } else {
            IndexedBlock::UndergroundBlock
        }
    }

    // pub fn get_block_at(&self, x: i16, y: u8, z: i16) -> Block {
    //     let palette = self.get_biome(x, z);
    //     let palette_block = self.get_indexed_block_at(x, y, z);
    //     palette.to_block(palette_block)
    // }

    pub fn get_biome(&self, _chunk_column: ChunkColumnCoordinates) -> biomes::Biome {
        biomes::Biome::Plains
    }

    pub fn get_palette(&self, chunk_coords: ChunkCoordinates) -> palettes::Palette {
        let biome = self.get_biome(chunk_coords.into());

        // 0 -> 0..15
        // 1 -> 16..31
        // 2 -> 32..47
        // 3 -> 48..63
        // 4 -> 64..79
        // 5 -> 80..95
        // 6 -> 96..111
        // 7 -> 112..127
        // 8 -> 128..143
        // 9 -> 144..159
        // 10 -> 160..175
        // 11 -> 176..191
        // 12 -> 192..207
        // 13 -> 208..223
        // 14 -> 224..239
        // 15 -> 240..255

        match chunk_coords.y {
            0..=1 => palettes::Palette::DeepUnderground,
            2..=3 => palettes::Palette::Underground,
            4..=11 => biome.into(),
            // TODO: 197..=255 => palettes::Palette::Air,
            _ => biome.into(),
        }
    }
}
