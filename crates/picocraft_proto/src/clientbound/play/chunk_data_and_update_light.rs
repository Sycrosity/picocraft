use crate::chunks::SkyLightSection;
use crate::prelude::*;

/// How many 16-block sections are in a column chunk.
const CHUNK_COLUMN_SECTIONS: usize = WORLD_HEIGHT / 16;

#[derive(Debug, Packet, bon::Builder)]
#[packet(id = 0x2c)]
pub struct ChunkDataAndUpdateLightPacket<Section: SkyLightSection> {
    pub chunk_x: Int,
    pub chunk_z: Int,
    pub data: ChunkData<36, 16, 256>,
    pub light: LightData<Section, { CHUNK_COLUMN_SECTIONS + 2 }>,
}
