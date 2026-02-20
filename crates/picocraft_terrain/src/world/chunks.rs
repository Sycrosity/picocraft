use picocraft_derive::{Decode, Encode, Packet};

use super::heightmaps::ChunkHeightmaps;
use super::palettes::Palette;
use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x2c)]
pub struct ChunkAndLightPacket {
    pub chunk_x: Int,
    pub chunk_z: Int,
    pub chunk_data: ChunkData,
    pub light_data: super::light::LightData,
}

#[derive(Debug)]
pub struct ChunkData {
    pub heightmaps: ChunkHeightmaps,

    pub data: Array<ChunkSection, 16>,
    pub block_entities: PrefixedArray<BlockEntity, 0>,
}

impl Encode for ChunkData {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        self.heightmaps.encode(&mut buffer).await?;

        let mut counter = ByteCountWriter::new();
        self.data.encode(&mut counter).await?;

        VarInt(counter.count as i32).encode(&mut buffer).await?; // should always be 33232 with this current setup
        log::info!("{}", counter.count);

        self.data.encode(&mut buffer).await?;

        self.block_entities.encode(&mut buffer).await?;

        Ok(())
    }
}

impl Decode for ChunkData {
    async fn decode<R: embedded_io_async::Read>(mut _buffer: R) -> Result<Self, DecodeError> {
        todo!()
    }
}

#[derive(Debug, Encode)]
pub struct ChunkSection {
    pub block_count: Short,
    pub blocks: BlockContainer,
    pub biomes: BiomeContainer,
}

#[derive(Debug)]
pub struct BlockContainer {
    pub bits_per_entry: UnsignedByte,
    pub palette: Palette,
    pub packed_blocks: Array<u64, 256>,
}

impl Encode for BlockContainer {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        self.bits_per_entry.encode(&mut buffer).await?;
        self.palette.encode(&mut buffer).await?;
        self.packed_blocks.encode(&mut buffer).await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BiomeContainer {
    // 3 bits per biome, so 21 biomes can fit in a single long. 4 longs are needed to represent all
    // 64 biomes in a chunk section.
    pub data: Array<u64, 4>,
}

impl Default for BiomeContainer {
    fn default() -> Self {
        Self {
            data: Array::from_array([0; 4]),
        }
    }
}

impl Encode for BiomeContainer {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        0u8.encode(&mut buffer).await?;

        0u8.encode(&mut buffer).await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct BlockEntity {
    packed_xz: UnsignedByte,
    y: u8,
    block_type: VarInt,
    data: BlockEntityData,
}

#[derive(Debug, Clone, Encode, Decode)]
struct BlockEntityData;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heightmap_set() {
        use super::*;

        let mut heightmap = Heightmap::new(HeightmapType::WorldSurface);

        heightmap.set(0, 0, Some(0));
        heightmap.set(1, 0, Some(1));
        heightmap.set(2, 0, Some(254));
        heightmap.set(3, 0, Some(254));
        heightmap.set(4, 0, Some(254));
        heightmap.set(5, 0, Some(254));
        heightmap.set(6, 0, Some(255));

        heightmap.set(14, 0, Some(254));
        heightmap.set(15, 0, Some(254));
        heightmap.set(0, 1, Some(254));
        heightmap.set(1, 1, Some(254));
        heightmap.set(2, 1, Some(254));
        heightmap.set(3, 1, Some(254));
        heightmap.set(4, 1, Some(254));

        use std::string::String;
        use std::vec::Vec;

        let collect = heightmap
            .data
            .iter()
            .map(|x| std::format!("{:064b}", x))
            .collect::<Vec<String>>();

        let tests = vec![
            String::from("0100000000011111111011111111011111111011111111000000010000000001"),
            String::from("0000000000000000000000000000000000000000000000000000000000000000"),
            String::from("0011111111011111111011111111011111111011111111011111111011111111"),
        ];

        for (i, test) in tests.iter().enumerate() {
            assert_eq!(
                &collect[i], test,
                "Heightmap data does not match expected value at Long #{}",
                i
            );
        }
    }
}
