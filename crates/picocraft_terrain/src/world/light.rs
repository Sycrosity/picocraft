use crate::prelude::*;

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
        PrefixedArray::from_array([FullLightSection; 18])
            .encode(&mut buffer)
            .await?;
        // block light arrays
        PrefixedArray::from_array([FullLightSection; 18])
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

#[derive(Debug, Clone, Copy)]
struct FullLightSection;

impl Encode for FullLightSection {
    // Equivelent to encoding a `PrefixedArray` of 2048 0xff values, aka all blocks
    // fully lit.
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        use picocraft_core::byteorder::WriteBytesExt;

        VarInt(2048).encode(&mut buffer).await?;

        for _ in 0..2048 {
            buffer.write_u8(0xff).await?;
        }

        Ok(())
    }
}
