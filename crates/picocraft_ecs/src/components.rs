use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Health(pub f32);

#[derive(Debug, Clone, Copy)]
pub struct OnGround;

#[derive(Debug, Clone)]
pub struct Username(pub String<16>);

/// Named "Motion" in Minecraft. Converted to f64 when serialised.
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Converted to f64 when serialised as minecraft uses f64 for positions, but
/// f32 is more than enough for internal use and allows FPU use on ESP32s3.
#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        f64::from(self.x)
    }

    pub fn y(&self) -> f64 {
        f64::from(self.y)
    }

    pub fn z(&self) -> f64 {
        f64::from(self.z)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rotation {
    /// Rotation around the vertical (y) axis, in degrees. From -180 to +180
    /// degrees. Increases when rotating to the right (clockwise), decreases
    /// when rotating to the left (counter-clockwise). 0 degrees means facing
    /// south.
    pub yaw: f32,
    /// Rotation around the local x axis, in degrees. From -90 to +90 degrees.
    /// Increases when looking downwards, decreases when looking up. 0 degrees
    /// means looking straight ahead, parallel to the ground. 80 degrees means
    /// looking straight down, -90 degrees means looking straight up.
    pub pitch: f32,
}

/// Converted to f64 when serialised.
#[derive(Debug, Clone, Copy)]
pub struct FallDistance(pub f32);

#[derive(Debug, Clone, Copy)]
pub struct Uuid(pub UUID);

/// A marker component for mobs that should not naturally despawn, such as pets
/// or bred animals.
#[derive(Debug, Clone, Copy)]
pub struct Persistent;

/// Realistically shouldn't be anything but the Overworld for now
#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}
