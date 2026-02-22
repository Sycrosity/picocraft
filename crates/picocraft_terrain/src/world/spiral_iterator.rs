use super::coordinates::ChunkColumnCoordinates;

pub struct BorderedSpiralIterator {
    inner: SpiralIterator,
    spawn_chunk: ChunkColumnCoordinates,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkKind {
    Terrain,
    Air,
}

impl BorderedSpiralIterator {
    /// Creates a spiral that covers `terrain_width + 2` columns (1 air border
    /// on each side).
    #[must_use]
    pub fn new(terrain_width: i8, spawn_chunk: ChunkColumnCoordinates) -> Self {
        let total_width = terrain_width + 2;

        Self {
            inner: SpiralIterator::new(total_width, spawn_chunk),
            spawn_chunk,
        }
    }

    fn is_terrain(&self, x: i8, z: i8) -> bool {
        let dx = x - self.spawn_chunk.x;
        let dz = z - self.spawn_chunk.z;

        let half_width = (self.inner.max_width / 2) - 1;

        dx >= -half_width && dx < half_width && dz >= -half_width && dz < half_width
    }
}

impl Iterator for BorderedSpiralIterator {
    type Item = (i8, i8, ChunkKind);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, z) = self.inner.next()?;
        let kind = if self.is_terrain(x, z) {
            ChunkKind::Terrain
        } else {
            ChunkKind::Air
        };
        Some((x, z, kind))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SprialStage {
    Center,
    Up,
    Right,
    Down,
    Left,
}

pub struct SpiralIterator {
    max_width: i8,
    spawn_chunk: ChunkColumnCoordinates,
    x: i8,
    z: i8,
    level: i8,
    stage: SprialStage,
}

impl SpiralIterator {
    #[must_use]
    pub fn new(max_width: i8, spawn_chunk: ChunkColumnCoordinates) -> Self {
        assert!(max_width > 0, "max_width must be greater than 0");
        assert!(
            max_width % 2 == 0,
            "max_width must be even, got {}",
            max_width
        );

        Self {
            max_width,
            spawn_chunk,
            x: spawn_chunk.x,
            z: spawn_chunk.z,
            level: 1,
            stage: SprialStage::Center,
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = (i8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.stage == SprialStage::Center {
            self.stage = SprialStage::Down;
            // return Some((self.spawn_chunk.x, self.spawn_chunk.z));
        }

        if self.level > self.max_width / 2 {
            return None;
        }

        let result = (self.x, self.z);

        // 16(-2, 1)  5(-1, 1)  6( 0, 1)  7( 1, 1)
        // 15(-2, 0)  4(-1, 0)  1( 0, 0)  8( 1, 0)
        // 14(-2,-1)  3(-1,-1)  2( 0,-1)  9( 1,-1)
        // 13(-2,-2) 12(-1,-2) 11( 0,-2) 10( 1,-2)
        match self.stage {
            SprialStage::Up => {
                self.z += 1;
                if self.z >= (self.level + self.spawn_chunk.z) {
                    self.stage = SprialStage::Right;
                    self.level += 1;
                }
            }
            SprialStage::Right => {
                self.x += 1;
                if self.x >= (self.level - 1 + self.spawn_chunk.x) {
                    self.stage = SprialStage::Down;
                }
            }
            SprialStage::Down => {
                self.z -= 1;
                if self.z <= -(self.level - self.spawn_chunk.z) {
                    self.stage = SprialStage::Left;
                }
            }
            SprialStage::Left => {
                self.x -= 1;
                if self.x <= -(self.level - self.spawn_chunk.x) {
                    self.stage = SprialStage::Up;
                }
            }
            _ => unreachable!(),
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::world::coordinates::ChunkColumnCoordinates;

    #[test]
    fn works_for_4() {
        use std::prelude::rust_2024::*;

        let spawn_chunk = ChunkColumnCoordinates::new(0, 0);
        let iterator = SpiralIterator::new(4, spawn_chunk);

        let expected = vec![
            (0, 0),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (1, -2),
            (0, -2),
            (-1, -2),
            (-2, -2),
            (-2, -1),
            (-2, 0),
            (-2, 1),
        ];

        let results = iterator.collect::<Vec<(i8, i8)>>();

        assert_eq!(results.len(), 16);

        assert_eq!(results, expected);
    }

    #[test]
    fn works_for_6() {
        use std::prelude::rust_2024::*;

        let spawn_chunk = ChunkColumnCoordinates::new(0, 0);
        let iterator = SpiralIterator::new(6, spawn_chunk);

        let expected = vec![
            (0, 0),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (1, -2),
            (0, -2),
            (-1, -2),
            (-2, -2),
            (-2, -1),
            (-2, 0),
            (-2, 1),
            (-2, 2),
            (-1, 2),
            (0, 2),
            (1, 2),
            (2, 2),
            (2, 1),
            (2, 0),
            (2, -1),
            (2, -2),
            (2, -3),
            (1, -3),
            (0, -3),
            (-1, -3),
            (-2, -3),
            (-3, -3),
            (-3, -2),
            (-3, -1),
            (-3, 0),
            (-3, 1),
            (-3, 2),
        ];

        let results = iterator.collect::<Vec<(i8, i8)>>();

        assert_eq!(results.len(), 36);

        assert_eq!(results, expected);
    }

    #[test]
    fn center_1_1() {
        use std::prelude::rust_2024::*;

        let spawn_chunk = ChunkColumnCoordinates::new(1, 1);
        let iterator = SpiralIterator::new(4, spawn_chunk);

        let expected = vec![
            (1, 1),
            (1, 0),
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (2, 1),
            (2, 0),
            (2, -1),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (-1, 2),
        ];

        let results = iterator.collect::<Vec<(i8, i8)>>();

        assert_eq!(results.len(), 16);

        assert_eq!(results, expected);
    }
}

#[test]
fn bordered_iterator_tags_correctly() {
    let spawn = ChunkColumnCoordinates::new(0, 0);
    let results: Vec<_> = BorderedSpiralIterator::new(4, spawn).collect();

    // terrain_width=4 -> terrain from -2..2 in both axes
    // total spiral width=6 → 36 chunks total
    assert_eq!(results.len(), 36);

    let terrain_count = results
        .iter()
        .filter(|(_, _, k)| *k == ChunkKind::Terrain)
        .count();
    let air_count = results
        .iter()
        .filter(|(_, _, k)| *k == ChunkKind::Air)
        .count();

    // 4*4 = 16 terrain, 36-16 = 20 air border
    assert_eq!(terrain_count, 16);
    assert_eq!(air_count, 20);
}
