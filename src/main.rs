use std::time::Duration;

use bevy_ecs::prelude::*;
use engine::drawing::draw_rectangle;
use engine::types::{Color, Rect, Vec2, Vec2F, VirtualKeyCode, WinitInputHelper};
use engine::{run, Context, Engine, GameState};

mod components;
use components::*;
mod resources;
use resources::*;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 240;

const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;

struct Game {
    ctx: Context,
    world: World,
    schedule: Schedule,
}

impl Game {
    pub fn new() -> Self {
        let ctx = Context {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            vsync_enabled: false,
        };
        let mut world = bevy_ecs::world::World::new();
        world.spawn().insert_bundle(CameraBundle::default());
        world.spawn().insert_bundle(PlayerBundle {position: Position { 0: Vec2F::new(1.0, 1.0) }, speed: Speed { 0: 7.0 }, ..Default::default()});
        world.insert_resource(Level::new_test());
        world.insert_resource(ControlBindings::default());
        let controls = world.get_resource::<ControlBindings>().unwrap();
        let mut movement_bindings: Vec<VirtualKeyCode> = Vec::with_capacity(8); // magic number, expects 2 per control,
        movement_bindings.extend(&controls.up);
        movement_bindings.extend(&controls.down);
        movement_bindings.extend(&controls.left);
        movement_bindings.extend(&controls.right);
        world.insert_resource(movement_bindings); // NOTE currently unused!
        let mut schedule = Schedule::default();
        schedule.add_stage(
            "update",
            SystemStage::parallel()
                .with_system(handle_player_movement)
                .with_system(handle_collision)
                .with_system(handle_player_camera)
        );

        Self {
            ctx,
            world,
            schedule,
        }
    }
}

fn any_key_held(input: &WinitInputHelper, keys: &[VirtualKeyCode]) -> bool {
    for key in keys {
        if input.key_held(*key) {
            return true;
        }
    }
    false
}

fn get_tile(x: i32, y: i32, level: &resources::Level) -> char {
    if x >= 0 && x < level.width as i32 && y >= 0 && y < level.height as i32 {
        level
            .repr
            .chars()
            .nth((y * level.width as i32 + x) as usize)
            .unwrap()
    } else {
        ' '
    }
}

fn handle_player_movement(
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

fn handle_collision(mut query: Query<(&mut Position, &mut Velocity)>, level: Res<Level>) {
    for (mut pos, mut vel) in &mut query {
        let level = level.as_ref();

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

fn handle_player_camera(
    mut camera_query: Query<(&mut Camera, &mut Position), (Without<Player>,)>,
    player_query: Query<(&Player, &Position), (Without<Camera>)>,
) {
    let (_, mut cam_pos) = camera_query.single_mut();
    let (_, player_pos) = player_query.single();
    cam_pos.0 = player_pos.0;
}

impl GameState for Game {
    fn on_update(&mut self, elapsed_time: Duration, engine: &mut Engine) -> bool {
        engine
            .window
            .set_title(&format!("{}ms", elapsed_time.as_millis()));
        let screen = &mut engine.screen;

        screen.clear(Color::new(50, 50, 193, 255));
        self.world.insert_resource(elapsed_time);
        self.world.insert_resource(engine.input.clone());
        self.schedule.run(&mut self.world);

        let (_, cam_pos) = self.world.query::<(&Camera, &Position)>().single(&self.world);
        let cam_pos = cam_pos.as_vec2f();
        let (_, player_pos) = self.world.query::<(&Player, &Position)>().single(&self.world);
        let player_pos = player_pos.as_vec2f();
        let level = self.world.resource::<Level>();

        let visible_tiles_x = SCREEN_WIDTH / TILE_WIDTH;
        let visible_tiles_y = SCREEN_HEIGHT / TILE_HEIGHT;

        let mut offset_x = cam_pos.x as f32 - visible_tiles_x as f32 / 2.0;
        let mut offset_y = cam_pos.y as f32 - visible_tiles_y as f32 / 2.0;

        if offset_x < 0.0 {
            offset_x = 0.0;
        } else if offset_x > (level.width - visible_tiles_x) as f32 {
            offset_x = (level.width - visible_tiles_x) as f32;
        }
        if offset_y < 0.0 {
            offset_y = 0.0;
        } else if offset_y > (level.height - visible_tiles_y) as f32 {
            offset_y = (level.height - visible_tiles_y) as f32;
        }

        let tile_offset_x = (offset_x - offset_x.trunc()) * TILE_WIDTH as f32;
        let tile_offset_y = (offset_y - offset_y.trunc()) * TILE_WIDTH as f32;

        for x in -1..(visible_tiles_x + 1) as i32 {
            for y in -1..(visible_tiles_y + 1) as i32 {
                let tile_id = get_tile(x + offset_x as i32, y + offset_y as i32, level);
                let rect = Rect::new(
                    Vec2::new(
                        x * TILE_WIDTH as i32 + 1 - tile_offset_x as i32,
                        y * TILE_WIDTH as i32 + 1 - tile_offset_y as i32,
                    ),
                    TILE_WIDTH - 1,
                    TILE_HEIGHT - 1,
                );
                match tile_id {
                    '.' => {
                        draw_rectangle(rect, screen, Color::new(255, 0, 255, 255));
                    }
                    '#' => {
                        draw_rectangle(rect, screen, Color::new(0, 255, 255, 255));
                    }
                    _ => {}
                };
            }
        }
        let player_rect = Rect::new(
            Vec2::new(
                ((player_pos.x - offset_x) * TILE_WIDTH as f32) as i32,
                ((player_pos.y - offset_y) * TILE_HEIGHT as f32) as i32,
            ),
            TILE_WIDTH,
            TILE_HEIGHT,
        );
        draw_rectangle(player_rect, screen, Color::new(255, 255, 255, 255));

        true
    }
    fn context(&self) -> &Context {
        &self.ctx
    }
}

fn main() {
    let game = Game::new();
    run(game);
}
