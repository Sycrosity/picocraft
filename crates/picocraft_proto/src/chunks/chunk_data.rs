use crate::prelude::*;

#[derive(Debug, Encode, Decode)]
pub struct ChunkData<const WORLD_HEIGHT: usize, const BYTES: usize, const BLOCKS: usize> {
    pub heightmaps: PrefixedArray<Heightmap<WORLD_HEIGHT>, 0>,
    pub data: PrefixedArray<Byte, BYTES>,
    pub block_entities: PrefixedArray<BlockEntities, BLOCKS>,
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

#[derive(Debug, Encode, Decode)]
pub struct BlockEntities {
    packed_xz: UnsignedByte,
    y: Short,
    block_type: VarInt,
    data: NBT,
}
