use std::collections::{HashMap, HashSet};

use engine::resource::ImageHandle;
use engine::types::{Rect, Vec2, Vec2F, VirtualKeyCode};

pub struct Level {
    pub name: String,
    pub spritesheet_handle: ImageHandle,
    pub dimensions: Vec2,
    pub background_tiles: HashMap<Vec2, Vec2>,
    pub foreground_tiles: HashMap<Vec2, Vec2>,
    pub collision: HashSet<Vec2>,
    pub entities: HashMap<Vec2, String>,
    pub bfs_flow_field_z: HashMap<Vec2, i16>,
}

impl Level {
    pub fn new(name: &str, handle: ImageHandle) -> LevelBuilder {
        LevelBuilder {
            name: name.to_owned(),
            spritesheet_handle: handle,
            background_tiles: None,
            foreground_tiles: None,
            collision: None,
            entities: None,
        }
    }
    pub fn reset_flow_field(&mut self) {
        self.bfs_flow_field_z =
            HashMap::from_iter(self.background_tiles.keys().map(|key| (*key, 0_i16)));
        for pos in &self.collision {
            self.bfs_flow_field_z.insert(*pos, -1);
        }
    }
}

#[derive(Clone)]
pub struct LevelBuilder {
    name: String,
    spritesheet_handle: ImageHandle,
    background_tiles: Option<HashMap<Vec2, Vec2>>,
    foreground_tiles: Option<HashMap<Vec2, Vec2>>,
    collision: Option<HashSet<Vec2>>,
    entities: Option<HashMap<Vec2, String>>,
}

impl LevelBuilder {
    pub fn background_tiles(mut self, tiles: HashMap<Vec2, Vec2>) -> Self {
        self.background_tiles = Some(tiles);
        self
    }
    pub fn foreground_tiles(mut self, tiles: HashMap<Vec2, Vec2>) -> Self {
        self.foreground_tiles = Some(tiles);
        self
    }
    pub fn collision(mut self, collision: HashSet<Vec2>) -> Self {
        self.collision = Some(collision);
        self
    }
    pub fn entities(mut self, entities: HashMap<Vec2, String>) -> Self {
        self.entities = Some(entities);
        self
    }
    pub fn build(&self) -> Level {
        if let Some(background_tiles) = self.background_tiles.clone() {
            let foreground_tiles = self.foreground_tiles.clone().unwrap_or_default();
            let tile_chain = background_tiles.keys().chain(foreground_tiles.keys());
            let min_x = tile_chain.clone().min_by_key(|v| v.x).unwrap();
            let min_y = tile_chain.clone().min_by_key(|v| v.y).unwrap();
            let max_x = tile_chain.clone().max_by_key(|v| v.x).unwrap();
            let max_y = tile_chain.clone().max_by_key(|v| v.y).unwrap();
            let width = max_x.x - min_x.x;
            let height = max_y.y - min_y.y;
            let dimensions = Vec2::new(width, height);
            let collision = self.collision.clone().unwrap_or_default();
            let entities = self.entities.clone().unwrap_or_default();
            let bfs_flow_field_z = HashMap::from_iter(
                self.background_tiles
                    .as_ref()
                    .unwrap()
                    .keys()
                    .map(|key| (*key, 0_i16)),
            );
            Level {
                name: self.name.clone(),
                spritesheet_handle: self.spritesheet_handle,
                dimensions,
                background_tiles,
                foreground_tiles,
                collision,
                entities,
                bfs_flow_field_z,
            }
        } else {
            panic!("LevelBuilder requires at least background tiles in order to be constructed");
        }
    }
}

pub struct ControlBindings {
    pub up: Vec<VirtualKeyCode>,
    pub down: Vec<VirtualKeyCode>,
    pub left: Vec<VirtualKeyCode>,
    pub right: Vec<VirtualKeyCode>,
}

impl Default for ControlBindings {
    fn default() -> Self {
        let up = vec![VirtualKeyCode::W, VirtualKeyCode::Up];
        let down = vec![VirtualKeyCode::S, VirtualKeyCode::Down];
        let left = vec![VirtualKeyCode::A, VirtualKeyCode::Left];
        let right = vec![VirtualKeyCode::D, VirtualKeyCode::Right];
        Self {
            up,
            down,
            left,
            right,
        }
    }
}

pub struct Screen {
    pub dim: Vec2,
}

pub struct TileMeta {
    pub dim: Vec2,
    pub visible: Vec2,
    pub offset: Vec2F,
}

pub struct MainMenuResources {
    pub button_1_handle: ImageHandle,
    pub button_1_bounds: Rect,
    pub button_quit_handle: ImageHandle,
    pub button_quit_bounds: Rect,
}
