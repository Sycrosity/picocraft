use picocraft_core::prelude::*;
use picocraft_derive::{Decode, Encode};

/// All heightmaps needed for a chunk packet.
#[derive(Debug)]
pub struct ChunkHeightmaps {
    /// Heightest non-air block/fluid in each column
    pub world_surface: Heightmap,
    /// Heightest block/fluid in each column that blocks motion ('solid' blocks,
    /// excl. bamboo saplings and cactuses) for displaying rain and snow
    pub motion_blocking: Heightmap,
    /// Same as motion_blocking, but also excludes leaves, used for mob spawning
    /// and pathfinding.
    pub motion_blocking_no_leaves: Heightmap,
}

impl Encode for ChunkHeightmaps {
    // This is the same as encoding a `PrefixedArray` of 3 `Heightmap`'s.
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        VarInt(3).encode(&mut buffer).await?;

        self.world_surface.encode(&mut buffer).await?;
        self.motion_blocking.encode(&mut buffer).await?;
        self.motion_blocking_no_leaves.encode(&mut buffer).await?;

        Ok(())
    }
}

impl Decode for ChunkHeightmaps {
    async fn decode<R: embedded_io_async::Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let heightmaps = PrefixedArray::<Heightmap, 3>::decode(&mut buffer).await?;

        let mut chunk_heightmaps = Self::new();

        for heightmap in heightmaps.iter() {
            match heightmap.heightmap_type {
                HeightmapType::WorldSurface => chunk_heightmaps.world_surface = heightmap.clone(),
                HeightmapType::MotionBlocking => {
                    chunk_heightmaps.motion_blocking = heightmap.clone()
                }
                HeightmapType::MotionBlockingNoLeaves => {
                    chunk_heightmaps.motion_blocking_no_leaves = heightmap.clone()
                }
            }
        }

        Ok(chunk_heightmaps)
    }
}

impl ChunkHeightmaps {
    #[must_use]
    pub fn new() -> Self {
        Self {
            world_surface: Heightmap::new(HeightmapType::WorldSurface),
            motion_blocking: Heightmap::new(HeightmapType::MotionBlocking),
            motion_blocking_no_leaves: Heightmap::new(HeightmapType::MotionBlockingNoLeaves),
        }
    }

    pub fn world_surface(&mut self) -> &mut Heightmap {
        &mut self.world_surface
    }

    pub fn motion_blocking(&mut self) -> &mut Heightmap {
        &mut self.motion_blocking
    }

    pub fn motion_blocking_no_leaves(&mut self) -> &mut Heightmap {
        &mut self.motion_blocking_no_leaves
    }
}

impl Default for ChunkHeightmaps {
    fn default() -> Self {
        Self::new()
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
#[derive(Debug, Clone, Encode, Decode, PartialEq)]
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

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq)]
#[protocol(value = VarInt)]
pub enum HeightmapType {
    WorldSurface = 1,
    MotionBlocking = 4,
    MotionBlockingNoLeaves = 5,
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
