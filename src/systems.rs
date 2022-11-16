use std::time::Duration;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ShouldRun;
use engine::types::{Vec2, Vec2F, VirtualKeyCode, WinitInputHelper};

use crate::components::*;
use crate::resources::*;
use crate::util::*;
use crate::{GameRunMode, ShouldQuit};

const BATCH_SIZE: usize = 1000;

pub fn is_in_game(state: Res<GameRunMode>) -> ShouldRun {
    match state.as_ref() {
        GameRunMode::Game => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

pub fn is_in_main_menu(state: Res<GameRunMode>) -> ShouldRun {
    match state.as_ref() {
        GameRunMode::MainMenu => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

pub fn handle_spacebar(input: Res<WinitInputHelper>, mut state: ResMut<GameRunMode>) {
    if input.key_pressed(VirtualKeyCode::Space) {
        *state = match *state {
            GameRunMode::MainMenu => GameRunMode::Game,
            GameRunMode::Game => GameRunMode::MainMenu,
        }
    }
}

pub fn handle_quit_button(input: Res<WinitInputHelper>, mmr: Res<MainMenuResources>, mut should_quit: ResMut<ShouldQuit>) {
    if input.mouse_pressed(0) {
        if let Some((x, y)) = input.mouse() {
            let resolution = input.resolution().unwrap();
            let (x, y) = resolution_to_screen_space(resolution, (x, y));
            if mmr.button_quit_bounds.point_intersects(Vec2::new(x as i32, y as i32)) {
                *should_quit = true;
            }
        }
    }
}

pub fn handle_enemy_movement(
    mut enemy_query: Query<(&Enemy, &Position, &mut Velocity, &Speed), Without<Player>>,
    player_query: Query<(&Player, &Position)>,
    elapsed_time: Res<Duration>,
) {
    let (_, player_pos) = player_query.single();
    enemy_query.par_for_each_mut(BATCH_SIZE, |(_, pos, mut vel, spd)| {
        let distance = f32::sqrt(f32::powi(player_pos.0.x - pos.0.x, 2) + f32::powi(player_pos.0.y - pos.0.y, 2));
        if distance < 3.5 && distance > 0.1 {
            vel.0.x = ((player_pos.0.x - pos.0.x) / distance) * 2.0;
            vel.0.y = ((player_pos.0.y - pos.0.y) / distance) * 2.0;
            vel.0 *= elapsed_time.as_secs_f32();
        } else {
            vel.0 *= 0.97 * elapsed_time.as_secs_f32();
        }
    });
}

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
    let friction = 3.999 * elapsed_time.as_secs_f32();
    vel.0 = vel.0 + ((target_velocity - vel.0) * friction);
    if !any_key_held(input, move_binds.as_ref()) {
        if vel.0.magnitude() < 0.001 {
            vel.0.x = 0.0;
            vel.0.y = 0.0;
        }
    }
}

pub fn handle_collision(mut query: Query<(&mut Position, &mut Velocity)>, level: Res<Level>) {
    let level = level.as_ref();
    query.par_for_each_mut(BATCH_SIZE, |(mut pos, mut vel)| {
        let mut new_position = pos.0 + vel.0;
        let leeway = 0.9_f32;

        // Collision handling
        if vel.0.x <= 0.0 {
            // moving left
            if level
                .collision
                .get(&Vec2::new(
                    new_position.x.floor() as i32,
                    pos.0.y.floor() as i32,
                ))
                .is_some()
                || level
                    .collision
                    .get(&Vec2::new(
                        new_position.x.floor() as i32,
                        (pos.0.y + leeway).floor() as i32,
                    ))
                    .is_some()
            {
                new_position.x = new_position.x.floor() + 1.0;
                vel.0.x = 0.0;
            }
        } else {
            // moving right
            if level
                .collision
                .get(&Vec2::new(
                    (new_position.x + 1.0).floor() as i32,
                    pos.0.y.floor() as i32,
                ))
                .is_some()
                || level
                    .collision
                    .get(&Vec2::new(
                        (new_position.x + 1.0).floor() as i32,
                        (pos.0.y + leeway).floor() as i32,
                    ))
                    .is_some()
            {
                new_position.x = new_position.x.floor();
                vel.0.x = 0.0;
            }
        }
        if vel.0.y <= 0.0 {
            // moving up
            if level
                .collision
                .get(&Vec2::new(
                    new_position.x.floor() as i32,
                    new_position.y.floor() as i32,
                ))
                .is_some()
                || level
                    .collision
                    .get(&Vec2::new(
                        (new_position.x + leeway).floor() as i32,
                        new_position.y.floor() as i32,
                    ))
                    .is_some()
            {
                new_position.y = new_position.y.floor() + 1.0;
                vel.0.y = 0.0;
            }
        } else {
            // moving down
            if level
                .collision
                .get(&Vec2::new(
                    new_position.x.floor() as i32,
                    (new_position.y + 1.0).floor() as i32,
                ))
                .is_some()
                || level
                    .collision
                    .get(&Vec2::new(
                        (new_position.x + leeway).floor() as i32,
                        (new_position.y + 1.0).floor() as i32,
                    ))
                    .is_some()
            {
                new_position.y = new_position.y.floor();
                vel.0.y = 0.0;
            }
        }
        pos.0 = new_position;
    });
}

pub fn handle_player_camera(
    mut camera_query: Query<(&mut Camera, &mut Position), Without<Player>>,
    player_query: Query<(&Player, &Position)>,
) {
    let (_, mut cam_pos) = camera_query.single_mut();
    let (_, player_pos) = player_query.single();
    cam_pos.0 = player_pos.0;
}

pub fn get_visible_tiles(screen: Res<Screen>, mut tile_meta: ResMut<TileMeta>) {
    tile_meta.visible.x = screen.dim.x / tile_meta.dim.x;
    tile_meta.visible.y = screen.dim.y / tile_meta.dim.y;
}

pub fn get_camera_offset(mut query: Query<(&mut Camera, &mut Position)>, tile_meta: Res<TileMeta>) {
    let (mut camera, pos) = query.single_mut();
    camera.offset.x = pos.0.x - tile_meta.visible.x as f32 / 2.0;
    camera.offset.y = pos.0.y - tile_meta.visible.y as f32 / 2.0;

    /*
    if offset_x < 0.0 {
        offset_x = 0.0;
    } else if offset_x > (level.dimensions.x - visible_tiles.x) as f32 {
        offset_x = (level.dimensions.x - visible_tiles.x) as f32;
    }
    if offset_y < 0.0 {
        offset_y = 0.0;
    } else if offset_y > (level.dimensions.y - visible_tiles.y) as f32 {
        offset_y = (level.dimensions.y - visible_tiles.y) as f32;
    }
    */
}

pub fn get_tile_offset(query: Query<&Camera>, mut tile_meta: ResMut<TileMeta>) {
    let camera = query.single();
    tile_meta.offset.x = (camera.offset.x - camera.offset.x.trunc()) * tile_meta.dim.x as f32;
    tile_meta.offset.y = (camera.offset.y - camera.offset.y.trunc()) * tile_meta.dim.y as f32;
}
