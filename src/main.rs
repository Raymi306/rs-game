use std::time::Duration;

use bevy_ecs::prelude::*;
use engine::types::{Color, Vec2, Vec2F, VirtualKeyCode};
use engine::{run, Context, Engine, GameState};

mod components;
use components::*;
mod resources;
use resources::*;
mod systems;
use systems::*;
mod render;
mod util;
use render::*;

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
        world.spawn().insert_bundle(PlayerBundle {
            position: Position {
                0: Vec2F::new(1.0, 1.0),
            },
            speed: Speed { 0: 7.0 },
            ..Default::default()
        });
        world.insert_resource(Level::new_test());
        world.insert_resource(ControlBindings::default());
        let controls = world.get_resource::<ControlBindings>().unwrap();
        let mut movement_bindings: Vec<VirtualKeyCode> = Vec::with_capacity(8); // magic number, expects 2 per control,
        movement_bindings.extend(&controls.up);
        movement_bindings.extend(&controls.down);
        movement_bindings.extend(&controls.left);
        movement_bindings.extend(&controls.right);
        world.insert_resource(movement_bindings);
        let mut schedule = Schedule::default();
        schedule.add_stage(
            "update",
            SystemStage::parallel()
                .with_system(handle_player_movement)
                .with_system(handle_collision)
                .with_system(handle_player_camera),
        );

        Self {
            ctx,
            world,
            schedule,
        }
    }
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

        let (_, cam_pos) = self
            .world
            .query::<(&Camera, &Position)>()
            .single(&self.world);
        let cam_pos = cam_pos.as_vec2f();
        let (_, player_pos) = self
            .world
            .query::<(&Player, &Position)>()
            .single(&self.world);
        let player_pos = player_pos.as_vec2f();
        let level = self.world.resource::<Level>();

        // Make new funcs const ----------------------------------------------
        let screen_dim = Vec2::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        let tile_dim = Vec2::new(TILE_WIDTH as i32, TILE_HEIGHT as i32);
        // -------------------------------------------------------------------
        // Could be systems? -------------------------------------------------
        let visible_tiles = get_visible_tiles(screen_dim, tile_dim);
        let camera_offset = get_camera_offset(cam_pos, visible_tiles, level);
        let tile_offset = get_tile_offset(camera_offset, TILE_WIDTH);
        // -------------------------------------------------------------------
        render_tiles(
            visible_tiles,
            camera_offset,
            tile_offset,
            &level,
            tile_dim,
            screen,
        );
        render_player(player_pos, camera_offset, tile_dim, screen);

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
