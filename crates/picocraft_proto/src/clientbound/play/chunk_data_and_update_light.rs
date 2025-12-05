use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x2c)]
pub struct ChunkDataAndUpdateLightPacket {
    chunk_x: Int,
    chunk_z: Int,
    data: ChunkData<256, 2048, 1024>,
    light: LightData<1024, 1024>,
}
