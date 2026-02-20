use crate::prelude::*;

#[derive(Debug, Encode)]
pub struct ChunkDataProto<const CHUNK_SECTIONS: usize, const BLOCK_TYPES: usize> {
    pub heightmaps: PrefixedArray<Heightmap, 3>,
    pub size: VarInt,
    // 512 is the number of longs needed to represent a 16x16x16 section with max 8 bits per block. 960 is the max vanilla can use, however.
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
/// Note: The reason the [`PrefixedArray`] is 37 [`u64`]'s long is that a
/// heightmap for a 16x16 block chunk needs 9 bits per height value (to cover
/// heights 0-256, aka worldheight+1 as the chunk could have no blocks in it).
/// Since the Long's have padding at the end of each long, 63 bits per long are
/// used, which is 7 heightmap values. Therefore, we need ceil(256/7) Long's,
/// which is 37. The 9 bits per entry also covers the current default world
/// height of 384 on vanilla servers, so the same logic applies if the world
/// height is increased in the future.
#[derive(Debug, Encode, Decode, PartialEq)]
pub struct Heightmap {
    pub heightmap_type: HeightmapType,
    pub data: PrefixedArray<u64, 37>,
}

impl Heightmap {
    pub fn new(heightmap_type: HeightmapType) -> Self {
        Self {
            heightmap_type,
            data: PrefixedArray::from_array([0; 37]),
        }
    }

    #[inline(always)]
    pub fn set(&mut self, x: u8, z: u8, height: Option<u8>) {
        let (x, z) = (x as usize, z as usize);
        let index = ((z * 16 + x) * 9) / 63;
        let bit_offset = ((z * 16 + x) * 9) % 63;

        let height = height.map(|h| u64::from(h) + 1).unwrap_or(0);

        // Clear value
        self.data[index] &= !(0x1ff << bit_offset);
        // Set value
        self.data[index] |= height << bit_offset;
    }
}

#[derive(Debug, Encode, Decode, PartialEq)]
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
    Direct,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct BlockEntitiesProto {
    packed_xz: UnsignedByte,
    y: Short,
    block_type: VarInt,
    data: NBT,
}

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
