use core::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug)]
pub struct ChunkCoordinates {
    x: i8,
    y: u8,
    z: i8,
}

impl ChunkCoordinates {
    pub fn new(x: i8, y: u8, z: i8) -> Self {
        assert!(y < 16, "Chunk y coordinate out of bounds: {y}");

        Self { x, y, z }
    }

    pub fn to_bounds(&self) -> CoordinateBounds {
        let start_x = (self.x as i16) * 16;

        //We probably shouldn't panic here, but I want to ensure there are no
        // overflows.
        let start_y = self.y.strict_mul(16);
        let start_z = (self.z as i16) * 16;

        CoordinateBounds::new(
            Coordinates::new((self.x as i16) * 16, self.y * 16, (self.z as i16) * 16),
            Coordinates::new(start_x + 15, start_y + 15, start_z + 15),
        )
    }
}

#[derive(Debug)]
pub struct CoordinateBounds {
    range_x: core::ops::RangeInclusive<i16>,
    range_y: core::ops::RangeInclusive<u8>,
    range_z: core::ops::RangeInclusive<i16>,
}

impl CoordinateBounds {
    pub fn new(start: Coordinates, end: Coordinates) -> Self {
        Self {
            range_x: (start.x)..=(end.x),
            range_y: (start.y)..=(end.y),
            range_z: (start.z)..=(end.z),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Coordinates> + '_ {
        self.range_y.clone().flat_map(move |y| {
            self.range_z
                .clone()
                .flat_map(move |z| self.range_x.clone().map(move |x| Coordinates::new(x, y, z)))
        })
    }
}

#[derive(Debug)]
pub struct Coordinates {
    pub x: i16,
    pub y: u8,
    pub z: i16,
}

impl Coordinates {
    pub fn new(x: i16, y: u8, z: i16) -> Self {
        Self { x, y, z }
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Coordinates {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
