use picocraft_derive::{Decode, Encode};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum IndexedBlock {
    Air,
    SurfaceBlock,
    SubSurfaceBlock,
    UndergroundBlock,

    DecorationBlock1,
    DecorationBlock2,
    DecorationBlock3,
    DecorationBlock4,

    FeatureBlock1,
    FeatureBlock2,
    FeatureBlock3,
    FeatureBlock4,

    Liquid,
    Special,
}

impl IndexedBlock {
    pub const ALL: [IndexedBlock; 14] = [
        IndexedBlock::Air,
        IndexedBlock::SurfaceBlock,
        IndexedBlock::SubSurfaceBlock,
        IndexedBlock::UndergroundBlock,
        IndexedBlock::DecorationBlock1,
        IndexedBlock::DecorationBlock2,
        IndexedBlock::DecorationBlock3,
        IndexedBlock::DecorationBlock4,
        IndexedBlock::FeatureBlock1,
        IndexedBlock::FeatureBlock2,
        IndexedBlock::FeatureBlock3,
        IndexedBlock::FeatureBlock4,
        IndexedBlock::Liquid,
        IndexedBlock::Special,
    ];

    pub fn get_id(&self) -> u8 {
        *self as u8
    }
}

/// This should really be generated from the `blocks.json` built-in registry.
#[derive(Debug, Encode, Decode)]
#[protocol(value = VarInt)]
pub enum Block {
    Air = 0,
    Dirt = 10,
    GrassBlock = 9,
    GrassBlockSnowy = 8,
    Gravel = 124,
    Sand = 118,
    Water = 86,
    Lava = 102,
    Obsidian = 3168,
    Bedrock = 85,

    Stone = 1,
    Deepslate = 27722,
    Granite = 2,
    Diorite = 4,
    Andesite = 6,
    Sandstone = 578,
    Mycelium = 8718,

    MossBlock = 27660,
    MossCarpet = 27611,

    CoalOre = 12711,
    IronOre = 131,
    GoldOre = 129,
    LapisOre = 563,
    CopperOre = 25111,
    DiamondOre = 5106,

    //Any negative values just havn't had their IDs assigned yet.
    DeepslateCoalOre = -1,
    DeepslateGoldOre = -2,
    DeepslateIronOre = -3,
    DeepslateLapisOre = -4,
    DeepslateDiamondOre = -5,
    DeepslateCopperOre = -6,

    OakLog = 137,
    SpruceLog = -8,
    BirchLog = -9,
    DarkOakLog = -10,

    OakLeaves = 279,
    SpruceLeaves = -12,
    BirchLeaves = -13,
    DarkOakLeaves = -14,

    OakSapling = -15,
    SpruceSapling = -16,
    BirchSapling = -17,
    DarkOakSapling = -18,

    Dandelion = 2121,
    Poppy = 2123,
    BlueOrchid = -21,
    Allium = 2125,
    LilyOfTheValley = -23,
    Cornflower = -24,
    RedMushroom = -25,
    BrownMushroom = 2135,

    Bush = -27,
    ShortDryGrass = -28,
    DeadBush = -29,
    Fern = -30,
    Vine = -31,
    Kelp = -32,
    SeaGrass = -33,
    SeaPickle = -34,
    ShortGrass = 2048,

    Wheat = -36,
    Carrots = -37,
    Potatos = -38,
    SweetBerryBush = -39,
    SugarCane = -40,
    Cactus = -41,

    Snow = -42,
    SnowBlock = -43,
    Clay = -44,
}
