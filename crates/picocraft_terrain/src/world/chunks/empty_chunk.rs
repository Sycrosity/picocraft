use crate::prelude::*;
use crate::world::biomes::Biome;
use crate::world::blocks::Block;
use crate::world::light::LightData;

#[derive(Debug, Clone, Copy)]
pub struct EmptyChunkAndLightPacket {
    chunk_x: Int,
    chunk_z: Int,
}

impl Packet for EmptyChunkAndLightPacket {
    const ID: VarInt = VarInt(0x2c);
    const STATE: State = State::Play;
}

impl core::fmt::Display for EmptyChunkAndLightPacket {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EmptyChunkAndLightPacket")
    }
}

impl EmptyChunkAndLightPacket {
    #[must_use]
    pub fn new(chunk_x: i8, chunk_z: i8) -> Self {
        Self {
            chunk_x: Int::from(chunk_x),
            chunk_z: Int::from(chunk_z),
        }
    }
}

impl Encode for EmptyChunkAndLightPacket {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        Self::ID.encode(&mut buffer).await?;

        self.chunk_x.encode(&mut buffer).await?;
        self.chunk_z.encode(&mut buffer).await?;

        // chunk_data
        EmptyChunkData.encode(&mut buffer).await?;

        // light_data
        LightData::empty().encode(&mut buffer).await?;

        Ok(())
    }
}

impl Decode for EmptyChunkAndLightPacket {
    async fn decode<R: embedded_io_async::Read>(_buffer: R) -> Result<Self, DecodeError> {
        unimplemented!("EmptyChunkAndLightPacket is never meant to be decoded")
    }
}

#[derive(Debug, Clone, Copy)]
struct EmptyChunkSection;

impl Encode for EmptyChunkSection {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        0i16.encode(&mut buffer).await?; // non-air block count
        0u8.encode(&mut buffer).await?; // bits per entry
        Block::Air.encode(&mut buffer).await?; // palette block value
        0u8.encode(&mut buffer).await?; // biome bits per entry
        Biome::Plains.encode(&mut buffer).await?; // biome palette value

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct EmptyChunkData;

impl Encode for EmptyChunkData {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        // heightmaps
        VarInt(0).encode(&mut buffer).await?;

        // length of data: 16 * (non-air block count + bits per entry + palette block
        // value + biome bits per entry + biome palette value). Might possibly
        // change if the plains biome is replaced with another (e.g. void) that has a
        // VarInt ID of over 0x8f

        VarInt(16 * (2 + 1 + 1 + 1 + 1)).encode(&mut buffer).await?;

        // chunk sections - "buffer"
        Array::<_, 16>::from_array([EmptyChunkSection; 16])
            .encode(&mut buffer)
            .await?;

        // block entities data
        VarInt(0).encode(&mut buffer).await?;

        Ok(())
    }
}
