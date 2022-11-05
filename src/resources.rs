use std::collections::{HashMap, HashSet};

use engine::resource::ImageHandle;
use engine::types::{Vec2, VirtualKeyCode};

pub struct Level {
    pub name: String,
    pub spritesheet_handle: ImageHandle,
    pub dimensions: Vec2,
    pub background_tiles: HashMap<Vec2, Vec2>,
    pub foreground_tiles: HashMap<Vec2, Vec2>,
    pub collision: HashSet<Vec2>,
    pub entities: HashMap<Vec2, String>,
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
            let foreground_tiles = self.foreground_tiles.clone().unwrap_or(HashMap::new());
            let tile_chain = background_tiles.keys().chain(foreground_tiles.keys());
            let min_x = tile_chain.clone().min_by_key(|v| v.x).unwrap();
            let min_y = tile_chain.clone().min_by_key(|v| v.y).unwrap();
            let max_x = tile_chain.clone().max_by_key(|v| v.x).unwrap();
            let max_y = tile_chain.clone().max_by_key(|v| v.y).unwrap();
            let width = max_x.x - min_x.x;
            let height = max_y.y - min_y.y;
            let dimensions = Vec2::new(width, height);
            let collision = self.collision.clone().unwrap_or(HashSet::new());
            let entities = self.entities.clone().unwrap_or(HashMap::new());
            Level {
                name: self.name.clone(),
                spritesheet_handle: self.spritesheet_handle,
                dimensions,
                background_tiles,
                foreground_tiles,
                collision,
                entities,
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
