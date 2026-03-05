use crate::Terrain;
use crate::noise::*;

#[non_exhaustive]
pub struct TerrainBuilder {
    seed: u64,
    sea_level: u8,
}

impl TerrainBuilder {
    #[must_use]
    pub fn new(seed: u64) -> TerrainBuilder {
        Self {
            seed,
            sea_level: 62,
        }
    }

    pub fn build(self) -> Terrain {
        Terrain {
            seed: self.seed,
            terrain_map: self.generate_terrain_map(),
            sea_level: self.sea_level,
        }
    }

    pub fn with_sea_level(mut self, sea_level: u8) -> Self {
        self.sea_level = sea_level;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    pub fn sea_level(&self) -> u8 {
        self.sea_level
    }

    pub fn set_sea_level(&mut self, sea_level: u8) {
        self.sea_level = sea_level;
    }

    fn generate_terrain_map(&self) -> NoiseMap256 {
        // let mut random =
        // rand_xoshiro::Xoroshiro128PlusPlus::seed_from_u64(self.seed);

        let perlin: FbmPerlin = FbmPerlin::new(self.seed() as u32).set_octaves(4);

        let mut terrain_map = NoiseMap256::default();

        //TODO this function is garbage.
        terrain_map.apply(|x, y| {
            (perlin.get([(x as f64 / 128.0 - 128.0), (y as f64 / 128.0 - 128.0)]) * 32.0 + 96.0)
                as u8
        });

        terrain_map
    }
}
