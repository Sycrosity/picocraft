use rand::distr::Uniform;
use rand::{Rng, SeedableRng};

use crate::noise_map::NoiseMap256;
use crate::prelude::*;

#[non_exhaustive]
pub struct World {
    seed: u64,
    terrain_map: NoiseMap256,
}

impl World {
    pub fn new(seed: u64) -> Self {
        let mut world = Self::initialise(seed);
        world.generate_terrain_map();
        world
    }

    pub fn initialise(seed: u64) -> Self {
        Self {
            seed,
            terrain_map: NoiseMap256::new(),
        }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn generate_terrain_map(&mut self) {
        let mut random = rand_xoshiro::Xoroshiro128PlusPlus::seed_from_u64(self.seed);

        // Example terrain generation using a simple noise function
        random.fill(self.terrain_map.map.as_flattened_mut());
    }

    #[cfg(feature = "images")]
    pub fn write_maps_to_folder<P: AsRef<std::path::Path>>(
        &self,
        folder: P,
    ) -> image::ImageResult<()> {
        self.terrain_map
            .write_to_image(folder.as_ref().join("terrain_map.png"))
    }
}
