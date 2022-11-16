use engine::Engine;
use engine::resource::{FontHandle, ImageHandle, ImageResource, Image};
use engine::types::{Color, Rect, Vec2};
use engine::constants::PIXEL_SIZE;
use engine::drawing::{draw_text_to_image, draw_rectangle_unchecked, blit_with_alpha};

use crate::SCREEN_WIDTH;

const PADDING: u32 = 4;
const WHITE: Color = Color::new(255, 255, 255, 255);

pub fn create_centered_button(engine: &mut Engine, font_handle: FontHandle, text: &str, y_off: i32) -> (ImageHandle, Rect) {
    let font = engine.resource_manager.get_font(font_handle).unwrap();
    let text_image = draw_text_to_image(
        font,
        &mut engine.font_helper.default_layout,
        text,
        20.0,
        WHITE,
    );
    let mut full_image_bounds = Rect::new(
        Vec2::new(0, 0),
        text_image.width() + PADDING * 2,
        text_image.height() + PADDING,
    );
    let full_image_buf: Vec<u8> = vec![0; (full_image_bounds.width * full_image_bounds.height * PIXEL_SIZE) as usize];
    let mut full_image = Image::new(full_image_bounds.width, full_image_bounds.height, full_image_buf);
    blit_with_alpha(&text_image, &mut full_image, Vec2::new(PADDING as i32, PADDING as i32));
    draw_rectangle_unchecked(
        Rect::new(
            Vec2::new(
                full_image_bounds.top_left.x + 1,
                full_image_bounds.top_left.y + 1,
            ),
            full_image_bounds.width - 2, full_image_bounds.height - 2),
        &mut full_image,
        Color::new(255, 255, 255, 255),
    );
    let center_x = (SCREEN_WIDTH / 2 - full_image.width() / 2) as i32;
    full_image_bounds.offset(Vec2::new(center_x, y_off));
    let result_handle = engine.resource_manager.add_image(full_image);
    (result_handle, full_image_bounds)
}
