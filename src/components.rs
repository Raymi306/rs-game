use bevy_ecs::prelude::*;
use engine::types::Vec2F;

macro_rules! as_vec2f {
    ($name : ident) => {
        impl $name {
            pub fn as_vec2f(&self) -> Vec2F {
                Vec2F::new(self.0.x, self.0.y)
            }
        }
    };
}

macro_rules! default_vec2f {
    ($name : ident) => {
        impl Default for $name {
            fn default() -> Self {
                Self {
                    0: Vec2F::new(0.0, 0.0)
                }
            }
        }
    }
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

#[derive(Component)]
pub struct Position(pub Vec2F);
as_vec2f!(Position);
default_vec2f!(Position);

#[derive(Component)]
pub struct Velocity(pub Vec2F);
as_vec2f!(Velocity);
default_vec2f!(Velocity);

#[derive(Component, Default)]
pub struct Speed(pub f32);
as_scalar!(Speed, f32);

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: Position,
    pub velocity: Velocity,
    pub speed: Speed,
}

#[derive(Component, Default)]
pub struct Camera;

#[derive(Bundle, Default)]
pub struct CameraBundle {
    camera: Camera,
    pub position: Position,
}
