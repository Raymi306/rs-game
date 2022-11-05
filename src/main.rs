use std::path::Path;
use std::time::Duration;

use bevy_ecs::prelude::*;
use engine::resource::Handle;
use engine::types::{Color, Vec2, VirtualKeyCode};
use engine::{run, Context, Engine, GameState};

mod components;
use components::*;
mod resources;
use resources::*;
mod systems;
use systems::*;
mod render;
use render::*;
mod file;
mod util;
use file::*;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const SCREEN_DIM: Vec2 = Vec2::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);

const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 32;
const TILE_DIM: Vec2 = Vec2::new(TILE_WIDTH as i32, TILE_HEIGHT as i32);

struct Game {
    ctx: Context,
    world: World,
    schedule: Schedule,
    handles: Vec<Handle>,
}

impl Game {
    pub fn new() -> Self {
        let ctx = Context {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            vsync_enabled: false,
        };
        let mut world = World::new();
        world.spawn().insert_bundle(CameraBundle::default());
        world.spawn().insert_bundle(PlayerBundle {
            position: Position::new(1.0, 1.0),
            speed: Speed::new(7.0),
            ..Default::default()
        });
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
            handles: Vec::new(),
        }
    }
}

impl GameState for Game {
    fn on_create(&mut self, engine: &mut Engine) -> bool {
        let handle = engine
            .resource_manager
            .load_image(Path::new("resources/images/level_1_spritesheet.png"));
        self.world.insert_resource(load_level(
            //Path::new("resources/maps/collision_test.lvl"),
            Path::new("resources/maps/level_1.lvl"),
            "Level 1",
            handle,
        ));
        self.handles.push(handle);
        true
    }
    fn on_update(&mut self, elapsed_time: Duration, engine: &mut Engine) -> bool {
        engine
            .window
            .set_title(&format!("{}ms", elapsed_time.as_millis()));
        self.world.insert_resource(elapsed_time);
        self.world.insert_resource(engine.input.clone());
        self.schedule.run(&mut self.world);
        {
            let screen = &mut engine.screen;
            screen.clear(Color::new(50, 50, 193, 255));
        }

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

        // Could be systems? -------------------------------------------------
        let visible_tiles = get_visible_tiles(SCREEN_DIM, TILE_DIM);
        let camera_offset = get_camera_offset(cam_pos, visible_tiles, level);
        let tile_offset = get_tile_offset(camera_offset, TILE_WIDTH);
        // -------------------------------------------------------------------
        render_tiles(
            visible_tiles,
            camera_offset,
            tile_offset,
            &level,
            TILE_DIM,
            engine,
        );
        let screen = &mut engine.screen;
        render_player(player_pos, camera_offset, TILE_DIM, screen);

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
