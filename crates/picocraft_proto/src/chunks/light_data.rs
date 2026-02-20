use crate::prelude::*;

#[derive(Debug, Encode, Decode)]
pub struct LightDataProto<Section: SkyLightSection, const SECTIONS: usize> {
    pub sky_light_mask: BitSet<1>,
    pub block_light_mask: BitSet<1>,
    pub empty_sky_light_mask: BitSet<1>,
    pub empty_block_light_mask: BitSet<1>,
    pub sky_light_arrays: PrefixedArray<Section, SECTIONS>,
    pub block_light_arrays: PrefixedArray<Section, SECTIONS>,
}

pub trait SkyLightSection: Encode + Decode + core::fmt::Debug {}

#[derive(Debug, Clone, Copy)]
pub struct FullSkyLightSection;

impl SkyLightSection for FullSkyLightSection {}

impl Encode for FullSkyLightSection {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        VarInt(2048).encode(&mut buffer).await?;

        for _ in 0..2048 {
            buffer.write_u8(0xff).await?;
        }

        Ok(())
    }
}

impl Decode for FullSkyLightSection {
    async fn decode<R: embedded_io_async::Read>(_buffer: R) -> Result<Self, DecodeError> {
        unimplemented!()
    }
}
