use std::path::Path;
use std::time::Duration;

use bevy_ecs::prelude::*;
use engine::types::{Color, FontSettings, Vec2, Vec2F, VirtualKeyCode};
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
use file::*;
mod menu;
use menu::*;
mod util;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 240;
const SCREEN_DIM: Vec2 = Vec2::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);

const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 32;
const TILE_DIM: Vec2 = Vec2::new(TILE_WIDTH as i32, TILE_HEIGHT as i32);

pub enum GameRunMode {
    MainMenu,
    Game,
}

pub type ShouldQuit = bool;

struct Game {
    ctx: Context,
    world: World,
    schedule: Schedule,
}

impl Game {
    fn new() -> Self {
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
        /*
        for i in 5..105 {
            world.spawn().insert_bundle(EnemyBundle {
                position: Position::new(i as f32, i as f32),
                ..Default::default()
            });
        }
        */
        world.spawn().insert_bundle(SmartEnemyBundle {
            enemy: EnemyBundle {
                position: Position::new(5.0, 5.0),
                ..Default::default()
            },
            ..Default::default()
        });
        world.insert_resource(ControlBindings::default());
        let controls = world.get_resource::<ControlBindings>().unwrap();
        let mut movement_bindings: Vec<VirtualKeyCode> = Vec::with_capacity(8); // magic number, expects 2 per control,
        movement_bindings.extend(&controls.up);
        movement_bindings.extend(&controls.down);
        movement_bindings.extend(&controls.left);
        movement_bindings.extend(&controls.right);
        let should_quit: ShouldQuit = false;
        world.insert_resource(should_quit);
        world.insert_resource(GameRunMode::MainMenu);
        world.insert_resource(movement_bindings);
        world.insert_resource(Screen { dim: SCREEN_DIM });
        world.insert_resource(TileMeta {
            dim: TILE_DIM,
            visible: Vec2::new(0, 0),
            offset: Vec2F::new(0.0, 0.0),
        });
        let mut schedule = Schedule::default();
        schedule.add_stage(
            "always",
            SystemStage::parallel().with_system(handle_spacebar),
        );
        schedule.add_stage(
            "update_menu",
            SystemStage::parallel()
                .with_run_criteria(is_in_main_menu)
                .with_system(handle_quit_button),
        );
        schedule.add_stage(
            "update_game",
            SystemStage::parallel()
                .with_run_criteria(is_in_game)
                .with_system(update_path_timers)
                .with_system(handle_player_movement)
                .with_system(update_player_trunc_pos)
                .with_system(propagate_pathfinding_wave.after(update_player_trunc_pos))
                .with_system(build_enemy_bfs_paths.after(propagate_pathfinding_wave))
                .with_system(handle_enemy_path_movement.after(build_enemy_bfs_paths))
                .with_system(handle_enemy_movement_dumb)
                .with_system(handle_collision)
                .with_system(handle_player_camera)
                .with_system(get_visible_tiles)
                .with_system(get_camera_offset.before(get_tile_offset))
                .with_system(get_tile_offset),
        );

        Self {
            ctx,
            world,
            schedule,
        }
    }
    fn game_create(&mut self, engine: &mut Engine) {
        let image_handle = engine
            .resource_manager
            .load_image(Path::new("resources/images/level_1_spritesheet.png"));
        self.world.insert_resource(load_level(
            Path::new("resources/maps/collision_test.lvl"),
            //Path::new("resources/maps/level_1.lvl"),
            "Level 1",
            image_handle,
        ));
    }
    fn game_update(&mut self, engine: &mut Engine) {
        let cam = self.world.query::<&Camera>().single(&self.world);
        let cam_offset = cam.offset;
        let (_, player_pos) = self
            .world
            .query::<(&Player, &Position)>()
            .single(&self.world);
        let player_pos = player_pos.as_wrapped();
        let level = self.world.resource::<Level>();
        let tile_meta = self.world.resource::<TileMeta>();
        {
            let screen = &mut engine.screen;
            screen.clear(Color::new(50, 50, 193, 255));
        }
        render_tiles(
            tile_meta.visible,
            cam_offset,
            tile_meta.offset,
            level,
            TILE_DIM,
            engine,
        );
        let screen = &mut engine.screen;
        let mut enemy_query = self.world.query::<(&Enemy, &Position)>();
        for (_enemy, pos) in enemy_query.iter(&self.world) {
            render_enemy(pos.0, cam_offset, TILE_DIM, screen);
        }
        render_player(player_pos, cam_offset, TILE_DIM, screen);
    }
    fn main_menu_create(&mut self, engine: &mut Engine) {
        let settings = FontSettings {
            scale: 10.0,
            ..FontSettings::default()
        };
        let font_handle = engine.resource_manager.load_font(
            Path::new("resources/fonts/JetbrainsMonoRegular.ttf"),
            settings,
        );
        let (button_1_handle, button_1_bounds) =
            create_centered_button(engine, font_handle, "Press Space to Toggle", 20);
        let (button_quit_handle, button_quit_bounds) =
            create_centered_button(engine, font_handle, "Quit", 70);
        self.world.insert_resource(MainMenuResources {
            button_1_handle,
            button_1_bounds,
            button_quit_handle,
            button_quit_bounds,
        });
    }
    fn main_menu_update(&mut self, engine: &mut Engine) {
        {
            let screen = &mut engine.screen;
            screen.clear(Color::new(13, 40, 183, 255));
        }
        let mmr = self.world.resource::<MainMenuResources>();
        render_main_menu(mmr, engine);
    }
}

impl GameState for Game {
    fn on_create(&mut self, engine: &mut Engine) -> bool {
        self.game_create(engine);
        self.main_menu_create(engine);

        true
    }
    fn on_update(&mut self, elapsed_time: Duration, engine: &mut Engine) -> bool {
        engine
            .window
            .set_title(&format!("{}ms", elapsed_time.as_millis()));
        self.world.insert_resource(elapsed_time);
        self.world.insert_resource(engine.input.clone());
        self.schedule.run(&mut self.world);
        let state = self.world.resource::<GameRunMode>();
        match state {
            GameRunMode::Game => self.game_update(engine),
            GameRunMode::MainMenu => self.main_menu_update(engine),
        };
        !*self.world.resource::<ShouldQuit>()
    }
    fn context(&self) -> &Context {
        &self.ctx
    }
}

fn main() {
    let game = Game::new();
    run(game);
}
