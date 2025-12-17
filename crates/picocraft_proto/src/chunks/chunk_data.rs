use crate::prelude::*;

#[derive(Debug, Encode)]
pub struct ChunkData<
    const WORLD_HEIGHT: usize,
    const CHUNK_SECTIONS: usize,
    const BLOCK_TYPES: usize,
> {
    pub heightmaps: PrefixedArray<Heightmap<WORLD_HEIGHT>, 0>,
    pub size: VarInt,
    pub data: Array<ChunkSection<BLOCK_TYPES>, CHUNK_SECTIONS>,
    pub block_entities: PrefixedArray<BlockEntities, 0>,
}

impl<const WORLD_HEIGHT: usize, const CHUNK_SECTIONS: usize, const BLOCK_TYPES: usize> Decode
    for ChunkData<WORLD_HEIGHT, CHUNK_SECTIONS, BLOCK_TYPES>
{
    async fn decode<R>(_buffer: R) -> Result<Self, DecodeError<R::Error>>
    where
        R: embedded_io_async::Read,
    {
        unimplemented!("Decoding ChunkData is not yet implemented")
    }
}

#[derive(Debug, Encode, Decode)]
pub struct Heightmap<const WORLD_HEIGHT: usize> {
    pub heighmap_type: HeightmapType,
    pub data: PrefixedArray<Long, WORLD_HEIGHT>,
}
#[derive(Debug, Encode, Decode)]
#[protocol(value = VarInt)]
pub enum HeightmapType {
    WorldSurface = 1,
    MotionBlocking = 4,
    MotionBlockingNoLeaves = 5,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ChunkSection<const BLOCK_TYPES: usize> {
    pub block_count: Short,
    pub block_states: BlockPalettedContainer<1024, BLOCK_TYPES>,
    pub biomes: BiomePalettedContainer<0, 0>,
}

#[derive(Debug, Clone)]
pub struct BlockPalettedContainer<const N: usize, const ENTRIES: usize> {
    pub bits_per_entry: UnsignedByte,
    pub palette: Palette<ENTRIES>,
    pub data: Array<Long, N>,
}

impl<const N: usize, const ENTRIES: usize> Encode for BlockPalettedContainer<N, ENTRIES> {
    async fn encode<W>(&self, mut buffer: W) -> Result<(), EncodeError<W::Error>>
    where
        W: embedded_io_async::Write,
    {
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
            _ => {
                log::warn!("Invalid bits_per_entry: {}", self.bits_per_entry);
                Err(EncodeError::Unknown)
            }
        }
    }
}

impl<const N: usize, const ELEMENTS: usize> Decode for BlockPalettedContainer<N, ELEMENTS> {
    async fn decode<R>(_buffer: R) -> Result<Self, DecodeError<R::Error>>
    where
        R: embedded_io_async::Read,
    {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct BiomePalettedContainer<const N: usize, const ENTRIES: usize> {
    pub bits_per_entry: UnsignedByte,
    pub palette: Palette<ENTRIES>,
    pub data: Array<Long, N>,
}

impl<const N: usize, const ENTRIES: usize> Encode for BiomePalettedContainer<N, ENTRIES> {
    async fn encode<W>(&self, mut buffer: W) -> Result<(), EncodeError<W::Error>>
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

impl<const N: usize, const ELEMENTS: usize> Decode for BiomePalettedContainer<N, ELEMENTS> {
    async fn decode<R>(_buffer: R) -> Result<Self, DecodeError<R::Error>>
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
pub struct BlockEntities {
    packed_xz: UnsignedByte,
    y: Short,
    block_type: VarInt,
    data: NBT,
}
