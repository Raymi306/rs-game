use std::time::Duration;

use bevy_ecs::prelude::*;
use engine::types::{Vec2F, VirtualKeyCode, WinitInputHelper};

use crate::components::*;
use crate::resources::*;
use crate::util::*;

pub fn handle_player_movement(
    mut query: Query<(&Player, &mut Velocity, &Speed)>,
    elapsed_time: Res<Duration>,
    input: Res<WinitInputHelper>,
    controls: Res<ControlBindings>,
    move_binds: Res<Vec<VirtualKeyCode>>,
) {
    let input = input.as_ref();
    let controls = controls.as_ref();
    let (_player, mut vel, spd) = query.single_mut();
    let spd = spd.as_scalar();
    let mut direction = Vec2F::new(0.0, 0.0);
    if any_key_held(input, &controls.up) {
        direction.y = -1.0;
    }
    if any_key_held(input, &controls.down) {
        direction.y = 1.0;
    }
    if any_key_held(input, &controls.left) {
        direction.x = -1.0;
    }
    if any_key_held(input, &controls.right) {
        direction.x = 1.0;
    }

    if direction.magnitude() > 1.0 {
        direction = direction.normalize();
    }

    let target_velocity = direction * spd * elapsed_time.as_secs_f32();
    let friction = 0.049;
    vel.0.x = vel.0.x + (target_velocity.x - vel.0.x) * friction;
    vel.0.y = vel.0.y + (target_velocity.y - vel.0.y) * friction;
    if !any_key_held(input, move_binds.as_ref()) {
        if f32::abs(vel.0.x) < 0.01 {
            vel.0.x = 0.0;
        }
        if f32::abs(vel.0.y) < 0.01 {
            vel.0.y = 0.0;
        }
    }
}

pub fn handle_collision(mut query: Query<(&mut Position, &mut Velocity)>, level: Res<Level>) {
    let level = level.as_ref();
    for (mut pos, mut vel) in &mut query {
        let mut new_position = Vec2F::new(0.0, 0.0);
        new_position.x = pos.0.x + vel.0.x;
        new_position.y = pos.0.y + vel.0.y;

        // Collision handling
        if vel.0.x <= 0.0 {
            // moving left
            if get_tile(new_position.x as i32, pos.0.y as i32, level) != '.'
                || get_tile(new_position.x as i32, (pos.0.y + 0.9) as i32, level) != '.'
            {
                new_position.x = new_position.x.trunc() + 1.0;
                vel.0.x = 0.0;
            }
        } else {
            // moving right
            if get_tile((new_position.x + 1.0) as i32, pos.0.y as i32, level) != '.'
                || get_tile((new_position.x + 1.0) as i32, (pos.0.y + 0.9) as i32, level) != '.'
            {
                new_position.x = new_position.x.trunc();
                vel.0.x = 0.0;
            }
        }
        if vel.0.y <= 0.0 {
            // moving up
            if get_tile(new_position.x as i32, new_position.y as i32, level) != '.'
                || get_tile((new_position.x + 0.9) as i32, new_position.y as i32, level) != '.'
            {
                new_position.y = new_position.y.trunc() + 1.0;
                vel.0.y = 0.0;
            }
        } else {
            // moving down
            if get_tile(new_position.x as i32, (new_position.y + 1.0) as i32, level) != '.'
                || get_tile(
                    (new_position.x + 0.9) as i32,
                    (new_position.y + 1.0) as i32,
                    level,
                ) != '.'
            {
                new_position.y = new_position.y.trunc();
                vel.0.y = 0.0;
            }
        }
        pos.0 = new_position;
    }
}

pub fn handle_player_camera(
    mut camera_query: Query<(&mut Camera, &mut Position), Without<Player>>,
    player_query: Query<(&Player, &Position)>,
) {
    let (_, mut cam_pos) = camera_query.single_mut();
    let (_, player_pos) = player_query.single();
    cam_pos.0 = player_pos.0;
}
