// pub struct NoiseMap2DBuilder<const SIZE: usize> {
//     map: NoiseMap2D<SIZE>,
// }

/// A 2D noise map with u8 values of size SIZE x SIZE. Must be a multiple of 16
/// so that it fits on chunk boundaries.
pub struct NoiseMap2D<const SIZE: usize> {
    /// The 2D array representing the noise map, stored as rows of u8 values.
    map: [[u8; SIZE]; SIZE],
}

impl<const SIZE: usize> NoiseMap2D<SIZE> {
    #[must_use]
    pub const fn new() -> Self {
        // Ensure SIZE is a multiple of 16 at compile time.
        const {
            assert!(SIZE.is_multiple_of(16));
        }

        Self {
            map: [[0; SIZE]; SIZE],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &u8> {
        self.map.iter().flat_map(|row| row.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut u8> {
        self.map.iter_mut().flat_map(|row| row.iter_mut())
    }

    pub fn rows(&self) -> impl Iterator<Item = &[u8; SIZE]> {
        self.map.iter()
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [u8; SIZE]> {
        self.map.iter_mut()
    }

    pub fn apply<F>(&mut self, mut generator: F)
    where
        F: FnMut(usize, usize) -> u8,
    {
        for y in 0..SIZE {
            for x in 0..SIZE {
                self.map[y][x] = generator(x, y);
            }
        }
    }

    pub fn from_generator<F>(mut generator: F) -> Self
    where
        F: FnMut(usize, usize) -> u8,
    {
        let mut map = Self::new();
        map.apply(&mut generator);
        map
    }

    #[inline]
    pub fn get(&self, x: i16, y: i16) -> Option<u8> {
        let y = y as usize;
        let x = x as usize;

        if y < SIZE && x < SIZE {
            Some(self.map[y][x])
        } else {
            None
        }
    }
}

impl<const SIZE: usize> Default for NoiseMap2D<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

/// A 256x256 noise map with u8 values.
pub type NoiseMap256 = NoiseMap2D<256>;
/// A 128x128 noise map with u8 values.
pub type NoiseMap128 = NoiseMap2D<128>;
/// A 64x64 noise map with u8 values.
pub type NoiseMap64 = NoiseMap2D<64>;
/// A 32x32 noise map with u8 values.
pub type NoiseMap32 = NoiseMap2D<32>;
