use crate::prelude::*;

#[derive(Debug, Encode, Decode)]
pub struct LightData<const SKYS: usize, const BLOCKS: usize> {
    sky_light_mask: BitSet<1>,
    block_light_mask: BitSet<1>,
    empty_sky_light_mask: BitSet<1>,
    empty_block_light_mask: BitSet<1>,
    sky_light_arrays: PrefixedArray<PrefixedArray<Byte, 2048>, SKYS>,
    block_light_arrays: PrefixedArray<PrefixedArray<Byte, 2048>, BLOCKS>,
}
