use engine::drawing::draw_rectangle;
use engine::types::{Color, Rect, Vec2, Vec2F};
use engine::Screen;

use crate::resources::Level;
use crate::util::*;

pub fn get_visible_tiles(screen_dim: Vec2, tile_dim: Vec2) -> Vec2 {
    let visible_tiles_x = screen_dim.x / tile_dim.x;
    let visible_tiles_y = screen_dim.y / tile_dim.y;
    Vec2::new(visible_tiles_x, visible_tiles_y)
}

pub fn get_camera_offset(cam_pos: Vec2F, visible_tiles: Vec2, level: &Level) -> Vec2F {
    let mut offset_x = cam_pos.x - visible_tiles.x as f32 / 2.0;
    let mut offset_y = cam_pos.y - visible_tiles.y as f32 / 2.0;

    if offset_x < 0.0 {
        offset_x = 0.0;
    } else if offset_x > (level.width - visible_tiles.x as u32) as f32 {
        offset_x = (level.width - visible_tiles.x as u32) as f32;
    }
    if offset_y < 0.0 {
        offset_y = 0.0;
    } else if offset_y > (level.height - visible_tiles.y as u32) as f32 {
        offset_y = (level.height - visible_tiles.y as u32) as f32;
    }
    Vec2F::new(offset_x, offset_y)
}

pub fn get_tile_offset(camera_offset: Vec2F, tile_width: u32) -> Vec2F {
    let tile_offset_x = (camera_offset.x - camera_offset.x.trunc()) * tile_width as f32;
    let tile_offset_y = (camera_offset.y - camera_offset.y.trunc()) * tile_width as f32;
    Vec2F::new(tile_offset_x, tile_offset_y)
}

pub fn render_tiles(
    visible_tiles: Vec2,
    camera_offset: Vec2F,
    tile_offset: Vec2F,
    level: &Level,
    tile_dim: Vec2,
    screen: &mut Screen,
) {
    for x in -1..(visible_tiles.x + 1) {
        for y in -1..(visible_tiles.y + 1) {
            let tile_id = get_tile(
                x + camera_offset.x as i32,
                y + camera_offset.y as i32,
                level,
            );
            let rect = Rect::new(
                Vec2::new(
                    x * tile_dim.x + 1 - tile_offset.x as i32,
                    y * tile_dim.x + 1 - tile_offset.y as i32,
                ),
                (tile_dim.x - 1) as u32,
                (tile_dim.y - 1) as u32,
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
}
pub fn render_player(player_pos: Vec2F, camera_offset: Vec2F, tile_dim: Vec2, screen: &mut Screen) {
    let player_rect = Rect::new(
        Vec2::new(
            ((player_pos.x - camera_offset.x) * tile_dim.x as f32) as i32,
            ((player_pos.y - camera_offset.y) * tile_dim.y as f32) as i32,
        ),
        tile_dim.x as u32,
        tile_dim.y as u32,
    );
    draw_rectangle(player_rect, screen, Color::new(255, 255, 255, 255));
}
