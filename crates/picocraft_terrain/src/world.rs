use crate::noise::*;
use crate::prelude::*;

#[non_exhaustive]
pub struct World {
    seed: u64,
    pub terrain_map: NoiseMap256,
}

impl World {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            terrain_map: NoiseMap256::new(),
        }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn generate_terrain_map(&mut self) {
        // let mut random =
        // rand_xoshiro::Xoroshiro128PlusPlus::seed_from_u64(self.seed);

        let perlin: FbmPerlin = FbmPerlin::new(self.seed() as u32).set_octaves(4);

        self.terrain_map.apply(|x, y| {
            (perlin.get([(x as f64 / 128.0 - 128.0), (y as f64 / 128.0 - 128.0)]) * 32.0 + 96.0)
                as u8
        });
    }
}
