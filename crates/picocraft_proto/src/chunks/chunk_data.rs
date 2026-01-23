use crate::prelude::*;

#[derive(Debug, Encode)]
pub struct ChunkDataProto<const CHUNK_SECTIONS: usize, const BLOCK_TYPES: usize> {
    pub heightmaps: PrefixedArray<Heightmap, 3>,
    pub size: VarInt,
    // 512 is the number of longs needed to represent a 16x16x16 section with max 8 bits per block.
    pub data: Array<ChunkSectionProto<512, BLOCK_TYPES>, CHUNK_SECTIONS>,
    pub block_entities: PrefixedArray<BlockEntitiesProto, 0>,
}

impl<const CHUNK_SECTIONS: usize, const BLOCK_TYPES: usize> Decode
    for ChunkDataProto<CHUNK_SECTIONS, BLOCK_TYPES>
{
    async fn decode<R>(_buffer: R) -> Result<Self, DecodeError>
    where
        R: embedded_io_async::Read,
    {
        unimplemented!("Decoding ChunkData is not yet implemented")
    }
}

/// Heightmap used in chunk data packets.
///
/// Note: The reason the Array is 36 Longs' long is that a heightmap for a 16x16
/// block chunk needs 9 bits per height value (to cover heights 0-256, aka
/// worldheight+1), and 16*16*9/64 = 36. The 9 bits per entry also covers the
/// current default world height of 384 on vanilla servers.
#[derive(Debug, Encode, Decode)]
pub struct Heightmap {
    pub heightmap_type: HeightmapType,
    pub data: PrefixedArray<Long, 36>,
}

#[derive(Debug, Encode, Decode)]
#[protocol(value = VarInt)]
pub enum HeightmapType {
    WorldSurface = 1,
    MotionBlocking = 4,
    MotionBlockingNoLeaves = 5,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ChunkSectionProto<const N: usize, const BLOCK_TYPES: usize> {
    pub block_count: Short,
    pub block_states: BlockPalettedContainerProto<N, BLOCK_TYPES>,
    pub biomes: BiomePalettedContainerProto<0, 0>,
}

#[derive(Debug, Clone)]
pub struct BlockPalettedContainerProto<const N: usize, const ENTRIES: usize> {
    pub bits_per_entry: UnsignedByte,
    pub palette: Palette<ENTRIES>,
    pub data: Array<Long, N>,
}

impl<const N: usize, const ENTRIES: usize> Encode for BlockPalettedContainerProto<N, ENTRIES> {
    async fn encode<W>(&self, mut buffer: W) -> Result<(), EncodeError>
    where
        W: embedded_io_async::Write,
    {
        if !matches!(self.bits_per_entry, 0 | 4..=8 | 15) {
            log::warn!("Invalid bits_per_entry: {}", self.bits_per_entry);
            return Err(EncodeError::InvalidBPE);
        }

        self.bits_per_entry.encode(&mut buffer).await?;

        match self.bits_per_entry {
            0 => {
                let Palette::SingleValued(block_id) = &self.palette else {
                    log::warn!("Palette type does not match bits_per_entry");
                    return Err(EncodeError::Unknown);
                };

                block_id.encode(&mut buffer).await?;

                // Direct encoding
                Ok(())
            }

            4..=8 => {
                let Palette::Indirect(palette) = &self.palette else {
                    log::warn!("Palette type does not match bits_per_entry");
                    return Err(EncodeError::Unknown);
                };

                palette.encode(&mut buffer).await?;
                self.data.encode(&mut buffer).await?;

                Ok(())
            }
            15 => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
}

impl<const N: usize, const ELEMENTS: usize> Decode for BlockPalettedContainerProto<N, ELEMENTS> {
    async fn decode<R>(_buffer: R) -> Result<Self, DecodeError>
    where
        R: embedded_io_async::Read,
    {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct BiomePalettedContainerProto<const N: usize, const ENTRIES: usize> {
    pub bits_per_entry: UnsignedByte,
    pub palette: Palette<ENTRIES>,
    pub data: Array<Long, N>,
}

impl<const N: usize, const ENTRIES: usize> Encode for BiomePalettedContainerProto<N, ENTRIES> {
    async fn encode<W>(&self, mut buffer: W) -> Result<(), EncodeError>
    where
        W: embedded_io_async::Write,
    {
        self.bits_per_entry.encode(&mut buffer).await?;

        match self.bits_per_entry {
            0 => {
                let Palette::SingleValued(biome_id) = &self.palette else {
                    log::warn!("Palette type does not match bits_per_entry");
                    return Err(EncodeError::Unknown);
                };

                biome_id.encode(&mut buffer).await?;

                // Direct encoding
                Ok(())
            }

            1..=3 => {
                unimplemented!(
                    "Encoding BiomePalettedContainer with bits_per_entry 1..=3 is not yet \
                     implemented"
                )
            }
            7 => {
                unimplemented!(
                    "Encoding BiomePalettedContainer with bits_per_entry 7 is not yet implemented"
                )
            }
            _ => {
                log::warn!("Invalid bits_per_entry: {}", self.bits_per_entry);
                Err(EncodeError::Unknown)
            }
        }
    }
}

impl<const N: usize, const ELEMENTS: usize> Decode for BiomePalettedContainerProto<N, ELEMENTS> {
    async fn decode<R>(_buffer: R) -> Result<Self, DecodeError>
    where
        R: embedded_io_async::Read,
    {
        unimplemented!("Decoding BiomePalettedContainer is not yet implemented")
    }
}

#[derive(Debug, Clone)]
pub enum Palette<const ENTRIES: usize> {
    SingleValued(VarInt),
    Indirect(PrefixedArray<VarInt, ENTRIES>),
    #[deprecated = "We should only have 8 bits of palette entries, so this variant is unnecessary"]
    Direct,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct BlockEntitiesProto {
    packed_xz: UnsignedByte,
    y: Short,
    block_type: VarInt,
    data: NBT,
}
