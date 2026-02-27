/// Marker trait for types that can be stored as ECS components.
///
/// Implement this for any data type that should be attached to entities.
pub trait Component: Sized + 'static {}

/// 3D position in the Minecraft world.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Component for Position {}

impl Position {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Velocity vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Component for Velocity {}

impl Velocity {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

/// Health points for living entities.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Component for Health {}

impl Health {
    pub const fn new(current: f32, max: f32) -> Self {
        Self { current, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}

/// Armour points that reduce incoming damage.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Armour {
    pub points: f32,
}

impl Component for Armour {}

/// Classifies an entity into a high-level kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Player,
    Zombie,
    Skeleton,
    Creeper,
    Spider,
    Item,
    FallingBlock,
}

impl Component for EntityKind {}

/// Tracks what a player is doing with blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlockInteraction {
    /// Player is breaking a block at (x, y, z).
    Breaking { x: i32, y: i32, z: i32 },
    /// Player is placing a block at (x, y, z).
    Placing { x: i32, y: i32, z: i32 },
    /// No active interaction.
    #[default]
    None,
}

impl Component for BlockInteraction {}

/// Pathfinding state for AI-controlled entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PathfindingState {
    pub target_x: i32,
    pub target_y: i32,
    pub target_z: i32,
    pub has_target: bool,
}

impl Component for PathfindingState {}
