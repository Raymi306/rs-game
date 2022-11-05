use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use engine::resource::ImageHandle;
use engine::types::Vec2;

use crate::resources::Level;

pub fn load_level(path: &Path, name: &str, spritesheet_handle: ImageHandle) -> Level {
    let mut background_tiles = HashMap::new();
    let mut foreground_tiles = HashMap::new();
    let mut collision_tiles = HashSet::new();
    let mut entities = HashMap::new();
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => {
            panic!("Couldn't create {}: {}", display, why);
        }
        Ok(f) => f,
    };
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let mut index = 0;
    let len_bg = u64::from_le_bytes(buf[index..index + 8].try_into().unwrap());
    index += 8;
    let len_fg = u64::from_le_bytes(buf[index..index + 8].try_into().unwrap());
    index += 8;
    let len_collision = u64::from_le_bytes(buf[index..index + 8].try_into().unwrap());
    index += 8;
    let bg_fg_stride_len = (8 + 8 + 2 + 2) as usize;
    let background_bytes = &buf[index..index + len_bg as usize];
    for chunk in background_bytes.chunks_exact(bg_fg_stride_len) {
        let x = i64::from_le_bytes(chunk[0..8].try_into().unwrap());
        let y = -i64::from_le_bytes(chunk[8..16].try_into().unwrap());
        let row = i16::from_le_bytes(chunk[16..18].try_into().unwrap());
        let col = i16::from_le_bytes(chunk[18..20].try_into().unwrap());
        background_tiles.insert(
            Vec2::new(x as i32, y as i32),
            Vec2::new(row as i32, col as i32),
        );
    }
    index += len_bg as usize;
    let foreground_bytes = &buf[index..index + len_fg as usize];
    for chunk in foreground_bytes.chunks_exact(bg_fg_stride_len) {
        let x = i64::from_le_bytes(chunk[0..8].try_into().unwrap());
        let y = -i64::from_le_bytes(chunk[8..16].try_into().unwrap());
        let row = i16::from_le_bytes(chunk[16..18].try_into().unwrap());
        let col = i16::from_le_bytes(chunk[18..20].try_into().unwrap());
        foreground_tiles.insert(
            Vec2::new(x as i32, y as i32),
            Vec2::new(row as i32, col as i32),
        );
    }
    index += len_fg as usize;
    let collision_stride_len = (8 + 8) as usize;
    let collision_bytes = &buf[index..index + len_collision as usize];
    for chunk in collision_bytes.chunks_exact(collision_stride_len) {
        let x = i64::from_le_bytes(chunk[0..8].try_into().unwrap());
        let y = -i64::from_le_bytes(chunk[8..16].try_into().unwrap());
        collision_tiles.insert(Vec2::new(x as i32, y as i32));
    }
    index += len_collision as usize;
    let len = buf.len();
    while index < len {
        let x = i64::from_le_bytes(buf[index..index + 8].try_into().unwrap());
        index += 8;
        let y = -i64::from_le_bytes(buf[index..index + 8].try_into().unwrap());
        index += 8;
        let label_len = u64::from_le_bytes(buf[index..index + 8].try_into().unwrap());
        index += 8;
        let label =
            String::from_utf8(buf[index..index + label_len as usize].try_into().unwrap()).unwrap();
        entities.insert(Vec2::new(x as i32, y as i32), label);
        index += label_len as usize;
    }
    let level_builder = Level::new(name, spritesheet_handle)
        .background_tiles(background_tiles)
        .foreground_tiles(foreground_tiles)
        .collision(collision_tiles)
        .entities(entities);
    level_builder.build()
}
