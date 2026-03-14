use crate::prelude::*;

// All biomes except plains currently don't work, since the shared registries
// aren't implemented well yet, and onlu plains is defined.
#[derive(Debug, picocraft_derive::Encode, Clone, Copy)]
#[protocol(value = VarInt)]
pub enum Biome {
    Plains = 0,
    Ocean = -1,
    Mountains = -2,
    Desert = -3,
    Taiga = -4,
    Savanna = -5,
    Forest = -6,
    River = -7,
}

impl Biome {
    pub const ALL: [Biome; 1] = [
        Biome::Plains,
        // Biome::Ocean,
        // Biome::Mountains,
        // Biome::Desert,
        // Biome::Taiga,
        // Biome::Savanna,
        // Biome::Forest,
        // Biome::River,
    ];

    pub fn index(&self) -> u8 {
        (*self) as u8
    }
}
