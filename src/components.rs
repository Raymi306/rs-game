use std::time::Duration;

use bevy_ecs::prelude::*;
use engine::timer::Timer;
use engine::types::{Vec2, Vec2F};

macro_rules! as_vec {
    ($name : ty, $wrapped : ty) => {
        impl $name {
            pub fn as_wrapped(&self) -> $wrapped {
                <$wrapped>::new(self.0.x, self.0.y)
            }
        }
    };
}

macro_rules! default_vec {
    ($name : ty, $wrapped : ty) => {
        impl Default for $name {
            fn default() -> Self {
                Self {
                    0: <$wrapped>::default(),
                }
            }
        }
    };
}

macro_rules! new_vec {
    ($name : ty, $wrapped : ty, $component : ty) => {
        impl $name {
            pub const fn new(x: $component, y: $component) -> Self {
                Self {
                    0: <$wrapped>::new(x, y),
                }
            }
        }
    };
}

macro_rules! as_scalar {
    ($name : ident, $type : ident) => {
        impl $name {
            pub fn as_scalar(&self) -> $type {
                self.0
            }
        }
    };
}

macro_rules! new_scalar {
    ($name : ident, $type : ident) => {
        impl $name {
            pub const fn new(val: $type) -> Self {
                Self { 0: val }
            }
        }
    };
}

#[derive(Component)]
pub struct Position(pub Vec2F);
as_vec!(Position, Vec2F);
default_vec!(Position, Vec2F);
new_vec!(Position, Vec2F, f32);

#[derive(Component)]
pub struct IntPosition(pub Vec2);
as_vec!(IntPosition, Vec2);
default_vec!(IntPosition, Vec2);
new_vec!(IntPosition, Vec2, i32);

#[derive(Component)]
pub struct Velocity(pub Vec2F);
as_vec!(Velocity, Vec2F);
default_vec!(Velocity, Vec2F);
new_vec!(Velocity, Vec2F, f32);

#[derive(Component, Default)]
pub struct Speed(pub f32);
as_scalar!(Speed, f32);
new_scalar!(Speed, f32);

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: Position,
    pub int_position: IntPosition,
    pub velocity: Velocity,
    pub speed: Speed,
}

#[derive(Component)]
pub struct Path {
    pub points: Vec<Vec2F>,
    pub next_point: Option<Vec2F>,
    pub timer: Timer,
}

impl Default for Path {
    fn default() -> Self {
        let timer = Timer::new(Duration::from_secs(1), true);
        let points = Vec::default();
        let next_point = None;
        Self {
            points,
            next_point,
            timer,
        }
    }
}

#[derive(Component, Default)]
pub struct AggroDistance(pub f32);
as_scalar!(AggroDistance, f32);
new_scalar!(AggroDistance, f32);

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct Dumb;

#[derive(Component, Default)]
pub struct Smart;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub position: Position,
    pub velocity: Velocity,
    pub speed: Speed,
    pub path: Path,
    pub aggro_distance: AggroDistance,
}

#[derive(Bundle, Default)]
pub struct SmartEnemyBundle {
    #[bundle]
    pub enemy: EnemyBundle,
    pub pathfinding: Smart,
}

#[derive(Bundle, Default)]
pub struct DumbEnemyBundle {
    #[bundle]
    pub enemy: EnemyBundle,
    pub pathfinding: Dumb,
}

#[derive(Component, Default)]
pub struct Camera {
    pub offset: Vec2F,
}

#[derive(Bundle, Default)]
pub struct CameraBundle {
    pub camera: Camera,
    pub position: Position,
}
