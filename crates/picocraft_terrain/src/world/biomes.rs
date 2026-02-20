use crate::prelude::*;
#[derive(Debug, picocraft_derive::Encode, Clone, Copy)]
#[protocol(value = VarInt)]
pub enum Biome {
    Plains = 0,
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

    pub fn palette_index(&self) -> u8 {
        (*self) as u8
    }
}
