mod map;

pub use map::{NoiseMap2D, NoiseMap32, NoiseMap64, NoiseMap128, NoiseMap256};

pub type FbmPerlin = noise::Fbm<noise::Perlin>;

pub use noise::{MultiFractal, NoiseFn, Perlin};

// use rand::distr::Uniform;
// use rand::{Rng, SeedableRng};
