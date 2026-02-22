use super::biomes::Biome;
use super::blocks::{Block, IndexedBlock};
use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Palette {
    Ocean,
    Plains,
    Forest,
    Mountains,
    Desert,
    Taiga,
    Savanna,
    River,
    Underground,
    DeepUnderground,
}

impl Encode for Palette {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        let map = IndexedBlock::ALL.map(|indexed_block| self.to_block(indexed_block));

        PrefixedArray::<_, 14>::from_array(map)
            .encode(&mut buffer)
            .await
    }
}

impl Palette {
    #[inline]
    pub fn to_block(&self, indexed_block: IndexedBlock) -> Block {
        match self {
            Palette::Plains => match indexed_block {
                IndexedBlock::Air => Block::Air,
                IndexedBlock::SurfaceBlock => Block::GrassBlock,
                IndexedBlock::SubSurfaceBlock => Block::Dirt,
                IndexedBlock::UndergroundBlock => Block::Stone,

                IndexedBlock::DecorationBlock1 => Block::Dandelion,
                IndexedBlock::DecorationBlock2 => Block::Poppy,
                IndexedBlock::DecorationBlock3 => Block::ShortGrass,
                IndexedBlock::DecorationBlock4 => Block::Allium,

                IndexedBlock::FeatureBlock1 => Block::OakLog,
                IndexedBlock::FeatureBlock2 => Block::OakLeaves,
                IndexedBlock::FeatureBlock3 => Block::MossCarpet,
                IndexedBlock::FeatureBlock4 => Block::BrownMushroom,
                IndexedBlock::Liquid => Block::Water,
                IndexedBlock::Special => Block::Lava,
                // _ => todo!(),
            },
            Palette::Ocean => match indexed_block {
                IndexedBlock::Air => Block::Air,
                IndexedBlock::SurfaceBlock => Block::Gravel,
                IndexedBlock::SubSurfaceBlock => Block::Gravel,
                IndexedBlock::UndergroundBlock => Block::Stone,

                IndexedBlock::DecorationBlock1 => Block::SeaGrass,
                IndexedBlock::DecorationBlock2 => Block::Kelp,
                IndexedBlock::DecorationBlock3 => Block::SeaPickle,
                // IndexedBlock::DecorationBlock4 => Block::,
                IndexedBlock::FeatureBlock1 => Block::Clay,
                IndexedBlock::FeatureBlock2 => Block::Sand,
                IndexedBlock::FeatureBlock3 => Block::Dirt,
                // IndexedBlock::FeatureBlock4 => Block::,
                IndexedBlock::Liquid => Block::Water,
                _ => todo!(),
            },
            Self::Forest => match indexed_block {
                IndexedBlock::Air => Block::Air,
                IndexedBlock::SurfaceBlock => Block::GrassBlock,
                IndexedBlock::SubSurfaceBlock => Block::Dirt,
                IndexedBlock::UndergroundBlock => Block::Stone,

                IndexedBlock::DecorationBlock1 => Block::Bush,
                IndexedBlock::DecorationBlock3 => Block::ShortGrass,
                IndexedBlock::DecorationBlock2 => Block::BrownMushroom,
                IndexedBlock::DecorationBlock4 => Block::MossCarpet,

                IndexedBlock::FeatureBlock1 => Block::OakLog,
                IndexedBlock::FeatureBlock2 => Block::OakLeaves,
                IndexedBlock::FeatureBlock3 => Block::BirchLog,
                IndexedBlock::FeatureBlock4 => Block::BirchLeaves,

                IndexedBlock::Liquid => Block::Water,
                IndexedBlock::Special => Block::Lava,
                // _ => todo!(),
            },
            Self::Mountains => match indexed_block {
                IndexedBlock::Air => Block::Air,
                IndexedBlock::SurfaceBlock => Block::GrassBlock,
                IndexedBlock::SubSurfaceBlock => Block::Dirt,
                IndexedBlock::UndergroundBlock => Block::Stone,

                IndexedBlock::DecorationBlock1 => Block::ShortGrass,
                IndexedBlock::DecorationBlock2 => Block::Snow,
                IndexedBlock::DecorationBlock3 => Block::SnowBlock,
                IndexedBlock::DecorationBlock4 => Block::SweetBerryBush,

                IndexedBlock::FeatureBlock1 => Block::Dirt,
                IndexedBlock::FeatureBlock2 => Block::IronOre,
                IndexedBlock::FeatureBlock3 => Block::CoalOre,
                IndexedBlock::FeatureBlock4 => Block::Gravel,

                IndexedBlock::Liquid => Block::Water,
                IndexedBlock::Special => Block::GrassBlockSnowy,
                // _ => todo!(),
            },
            Self::Desert => match indexed_block {
                IndexedBlock::Air => Block::Air,
                IndexedBlock::SurfaceBlock => Block::Sand,
                IndexedBlock::SubSurfaceBlock => Block::Sandstone,
                IndexedBlock::UndergroundBlock => Block::Stone,

                IndexedBlock::DecorationBlock1 => Block::Cactus,
                IndexedBlock::DecorationBlock2 => Block::DeadBush,
                IndexedBlock::DecorationBlock3 => Block::ShortDryGrass,
                // IndexedBlock::DecorationBlock4 => Block::

                // IndexedBlock::FeatureBlock1 => Block::,
                // IndexedBlock::FeatureBlock2 => Block::,
                // IndexedBlock::FeatureBlock3 => Block::,
                // IndexedBlock::FeatureBlock4 => Block::,
                IndexedBlock::Liquid => Block::Water,
                _ => todo!(),
            },

            Self::Taiga => match indexed_block {
                IndexedBlock::Air => Block::Air,
                IndexedBlock::SurfaceBlock => Block::GrassBlock,
                IndexedBlock::SubSurfaceBlock => Block::Dirt,
                IndexedBlock::UndergroundBlock => Block::Stone,

                IndexedBlock::DecorationBlock1 => Block::Fern,
                IndexedBlock::DecorationBlock2 => Block::Bush,
                IndexedBlock::DecorationBlock3 => Block::Cornflower,
                IndexedBlock::DecorationBlock4 => Block::BlueOrchid,

                IndexedBlock::FeatureBlock1 => Block::SpruceLog,
                IndexedBlock::FeatureBlock2 => Block::SpruceLeaves,
                IndexedBlock::FeatureBlock3 => Block::Mycelium,
                // IndexedBlock::FeatureBlock4 => Block::,
                IndexedBlock::Liquid => Block::Water,
                _ => todo!(),
            },
            Self::Savanna => todo!(),
            Self::River => todo!(),
            Self::Underground => todo!(),
            Self::DeepUnderground => todo!(),
        }
    }
}

impl From<Biome> for Palette {
    fn from(biome: Biome) -> Self {
        match biome {
            Biome::Plains => Palette::Plains,
            Biome::Ocean => Palette::Ocean,
            Biome::Mountains => Palette::Mountains,
            Biome::Desert => Palette::Desert,
            Biome::Taiga => Palette::Taiga,
            Biome::Savanna => Palette::Savanna,
            Biome::Forest => Palette::Forest,
            Biome::River => Palette::River,
        }
    }
}
