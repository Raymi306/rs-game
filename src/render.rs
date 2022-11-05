use engine::drawing::{blit_rect, blit_rect_with_alpha, draw_rectangle, draw_text};
use engine::types::{Color, Rect, Vec2, Vec2F};
use engine::{Engine, Screen};

use crate::resources::*;

pub fn render_tiles(
    visible_tiles: Vec2,
    camera_offset: Vec2F,
    tile_offset: Vec2F,
    level: &Level,
    tile_dim: Vec2,
    engine: &mut Engine,
) {
    let spritesheet = engine
        .resource_manager
        .get_image(level.spritesheet_handle)
        .unwrap();
    for x in -1..(visible_tiles.x + 1) {
        for y in -1..(visible_tiles.y + 1) {
            if let Some(tile) = level.background_tiles.get(&Vec2::new(
                x + camera_offset.x as i32,
                y + camera_offset.y as i32,
            )) {
                let src_rect = Rect::new(
                    Vec2::new(tile.x, tile.y),
                    tile_dim.x as u32,
                    tile_dim.y as u32,
                );
                let pos = Vec2::new(
                    x * tile_dim.x - tile_offset.x as i32,
                    y * tile_dim.y - tile_offset.y as i32,
                );
                blit_rect(spritesheet, src_rect, &mut engine.screen, pos);
            }
        }
    }
    for x in -1..(visible_tiles.x + 1) {
        for y in -1..(visible_tiles.y + 1) {
            if let Some(tile) = level.foreground_tiles.get(&Vec2::new(
                x + camera_offset.x as i32,
                y + camera_offset.y as i32,
            )) {
                let src_rect = Rect::new(
                    Vec2::new(tile.x, tile.y),
                    tile_dim.x as u32,
                    tile_dim.y as u32,
                );
                let pos = Vec2::new(
                    x * tile_dim.x - tile_offset.x as i32,
                    y * tile_dim.y - tile_offset.y as i32,
                );
                blit_rect_with_alpha(spritesheet, src_rect, &mut engine.screen, pos);
            }
        }
    }
}
pub fn render_collision(
    visible_tiles: Vec2,
    camera_offset: Vec2F,
    tile_offset: Vec2F,
    level: &Level,
    tile_dim: Vec2,
    engine: &mut Engine,
) {
    for x in -1..(visible_tiles.x + 1) {
        for y in -1..(visible_tiles.y + 1) {
            if level
                .collision
                .get(&Vec2::new(
                    x + camera_offset.x as i32,
                    y + camera_offset.y as i32,
                ))
                .is_some()
            {
                let collision_rect = Rect::new(
                    Vec2::new(
                        (x as f32 * tile_dim.x as f32 - tile_offset.x) as i32,
                        (y as f32 * tile_dim.y as f32 - tile_offset.y) as i32,
                    ),
                    tile_dim.x as u32,
                    tile_dim.y as u32,
                );
                draw_rectangle(
                    collision_rect,
                    &mut engine.screen,
                    Color::new(255, 0, 0, 255),
                );
            }
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

pub fn render_main_menu(resources: &MainMenuResources, engine: &mut Engine) {
    let font = engine
        .resource_manager
        .get_font(resources.font_handle)
        .unwrap();
    draw_text(
        font,
        &mut engine.font_helper.default_layout,
        "Press Spacebar to Toggle Modes",
        20.0,
        Color::new(255, 255, 255, 255),
        &mut engine.screen,
        Vec2 { x: 20, y: 20 },
    );
}
