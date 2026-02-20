use picocraft_derive::{Decode, Encode, Packet};
use picocraft_proto::chunks::{
    BlockEntitiesProto, FullSkyLightSection, Heightmap, HeightmapType,
};

use super::palettes::Palette;
use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x2c)]
pub struct ChunkAndLightPacket {
    pub chunk_x: Int,
    pub chunk_z: Int,
    pub chunk_data: ChunkData,
    pub light_data: LightData,
}

#[derive(Debug)]
pub struct ChunkData {
    pub heightmaps: ChunkHeightmaps,

    pub data: Array<ChunkSection, 16>,
    pub block_entities: PrefixedArray<BlockEntitiesProto, 0>,
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

#[derive(Debug)]
pub struct LightData;

impl Encode for LightData {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        //sky light mask

        BitSet::<1>::from_array([0x3ffff])
            .encode(&mut buffer)
            .await?;
        // block light mask
        BitSet::<1>::from_array([0x3ffff])
            .encode(&mut buffer)
            .await?;
        // empty sky light mask
        BitSet::<1>::new().encode(&mut buffer).await?;
        // empty block light mask
        BitSet::<1>::new().encode(&mut buffer).await?;

        // sky light arrays
        PrefixedArray::from_array([FullSkyLightSection; 18])
            .encode(&mut buffer)
            .await?;
        // block light arrays
        PrefixedArray::from_array([FullSkyLightSection; 18])
            .encode(&mut buffer)
            .await?;

        Ok(())
    }
}

impl Decode for LightData {
    async fn decode<R: embedded_io_async::Read>(_buffer: R) -> Result<Self, DecodeError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ChunkHeightmaps {
    pub world_surface: Heightmap,
    pub motion_blocking: Heightmap,
    pub motion_blocking_no_leaves: Heightmap,
}

impl Encode for ChunkHeightmaps {
    // This is the same as encoding a PrefixedArray of 3 `Heightmap`'s.
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        VarInt(3).encode(&mut buffer).await?;

        self.world_surface.encode(&mut buffer).await?;
        self.motion_blocking.encode(&mut buffer).await?;
        self.motion_blocking_no_leaves.encode(&mut buffer).await?;

        Ok(())
    }
}

impl ChunkHeightmaps {
    #[must_use]
    pub fn new() -> Self {
        Self {
            world_surface: Heightmap::new(HeightmapType::WorldSurface),
            motion_blocking: Heightmap::new(HeightmapType::MotionBlocking),
            motion_blocking_no_leaves: Heightmap::new(HeightmapType::MotionBlockingNoLeaves),
        }
    }

    pub fn world_surface(&mut self) -> &mut Heightmap {
        &mut self.world_surface
    }

    pub fn motion_blocking(&mut self) -> &mut Heightmap {
        &mut self.motion_blocking
    }

    pub fn motion_blocking_no_leaves(&mut self) -> &mut Heightmap {
        &mut self.motion_blocking_no_leaves
    }
}

impl Default for ChunkHeightmaps {
    fn default() -> Self {
        Self::new()
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

// #[derive(Debug)]
// pub struct BlockEntity {
//     packed_xz: UnsignedByte,
//     y: u8,
//     block_type: VarInt,
//     data: BlockEntityData,
// }
