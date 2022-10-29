use std::time::Duration;

use engine::{
    Context, Engine, GameState, run
};
use engine::types::{Color, Rect, Vec2, Vec2F, VirtualKeyCode};
use engine::drawing::draw_rectangle;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 240;

struct Game {
    ctx: Context,
    level: Level,
    camera: Camera,
    player: Player,
}

struct Level {
    width: u32,
    height: u32,
    repr: String,
}

impl Default for Level {
    fn default() -> Self {
        let width = 64;
        let height = 16;
        let mut repr = "".to_owned();
        repr += "................................................................";
        repr += "................................................................";
        repr += "................................................................";
        repr += "................................................................";
        repr += "................................................................";
        repr += "................................................................";
        repr += "........................#.......................................";
        repr += ".......................##.......................................";
        repr += ".........#............###.###.##...............................#";
        repr += ".....................####.......................................";
        repr += "....................#####.......................................";
        repr += "###########################.#######.############################";
        repr += "..........................#.#.....#.#...........................";
        repr += "..........................#.#######.#...........................";
        repr += "..........................#.........#...........................";
        repr += "..........................###########...........................";
        Self { width, height, repr }
    }
}

struct Camera {
    pos: Vec2F,
}

impl Default for Camera {
    fn default() -> Self {
        Self {pos: Vec2F::new(0.0, 0.0)}
    }
}

struct Player {
    pos: Vec2F,
    vel: Vec2F,
    speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Vec2F::new(0.0, 0.0),
            vel: Vec2F::new(0.0, 0.0),
            speed: 6.0,
        }
    }
}

const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;

impl Game {
    pub fn new() -> Self {
        let ctx = Context {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            vsync_enabled: false,
        };
        let level = Level::default();
        let camera = Camera::default();
        let player = Player::default();
        Self { ctx, level, camera, player }
    }
}

impl Game {
    #[inline(always)]
    fn get_tile(&self, x: i32, y: i32) -> char {
        if x >= 0 && x < self.level.width as i32 && y >= 0 && y < self.level.height as i32{
            self.level.repr.chars().nth((y * self.level.width as i32 + x) as usize).unwrap()
        } else {
            ' '
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

        if engine.input.key_held(VirtualKeyCode::Up) {
            self.player.vel.y = -1.0;
        }
        if engine.input.key_held(VirtualKeyCode::Down) {
            self.player.vel.y = 1.0;
        }
        if engine.input.key_held(VirtualKeyCode::Left) {
            self.player.vel.x = -1.0;
        }
        if engine.input.key_held(VirtualKeyCode::Right) {
            self.player.vel.x = 1.0;
        }

        println!("{:?}", self.player.vel);
        self.player.vel = self.player.vel.normalize() * self.player.speed;

        /*
        self.player.vel.x += -4.0 * self.player.vel.x * elapsed_time.as_secs_f32();
        if f32::abs(self.player.vel.x) < 0.13 {
            self.player.vel.x = 0.0;
        }
        self.player.vel.y += -4.0 * self.player.vel.y * elapsed_time.as_secs_f32();
        if f32::abs(self.player.vel.y) < 0.13 {
            self.player.vel.y = 0.0;
        }
        */
        println!("{:?}", self.player.vel);

        self.player.pos.x += self.player.vel.x * elapsed_time.as_secs_f32();
        self.player.pos.y += self.player.vel.y * elapsed_time.as_secs_f32();
        
        self.camera.pos.x = self.player.pos.x;
        self.camera.pos.y = self.player.pos.y;

        let visible_tiles_x = SCREEN_WIDTH / TILE_WIDTH;
        let visible_tiles_y = SCREEN_HEIGHT / TILE_HEIGHT;

        let mut offset_x = self.camera.pos.x as f32 - visible_tiles_x as f32 / 2.0;
        let mut offset_y = self.camera.pos.y as f32 - visible_tiles_y as f32 / 2.0;

        if offset_x < 0.0 {
            offset_x = 0.0;
        } else if offset_x > (self.level.width - visible_tiles_x) as f32 {
            offset_x = (self.level.width - visible_tiles_x) as f32;
        }
        if offset_y < 0.0 {
            offset_y = 0.0;
        } else if offset_y > (self.level.height - visible_tiles_y) as f32 {
            offset_y = (self.level.height - visible_tiles_y) as f32;
        }

        let tile_offset_x = (offset_x - offset_x.trunc()) * TILE_WIDTH as f32;
        let tile_offset_y = (offset_y - offset_y.trunc()) * TILE_WIDTH as f32;
        
        for x in -1..(visible_tiles_x + 1) as i32 {
            for y in -1..(visible_tiles_y + 1) as i32 {
                let tile_id = self.get_tile(x + offset_x as i32, y + offset_y as i32);
                let rect = Rect::new(
                    Vec2::new(
                        x * TILE_WIDTH as i32 + 1 - tile_offset_x as i32,
                        y * TILE_WIDTH as i32 + 1 - tile_offset_y as i32,
                    ),
                    TILE_WIDTH - 1,
                    TILE_HEIGHT - 1
                );
                match tile_id {
                    '.' => {
                        draw_rectangle(rect, screen, Color::new(255, 0, 255, 255));
                    },
                    '#' => {
                        draw_rectangle(rect, screen, Color::new(0, 255, 255, 255));
                    },
                    _ => {},
                };
            }
        }
        let player_rect = Rect::new(
            Vec2::new(
                ((self.player.pos.x - offset_x) * TILE_WIDTH as f32) as i32,
                ((self.player.pos.y - offset_y) * TILE_HEIGHT as f32) as i32,
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
