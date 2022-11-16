use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use engine::types::{VirtualKeyCode, WinitInputHelper};

pub fn any_key_held(input: &WinitInputHelper, keys: &[VirtualKeyCode]) -> bool {
    for key in keys {
        if input.key_held(*key) {
            return true;
        }
    }
    false
}

pub fn resolution_to_screen_space(resolution: (u32, u32), point: (f32, f32)) -> (u32, u32) {
    let resolution = (resolution.0 as f32, resolution.1 as f32);
    let x_sidebar = resolution.0 % SCREEN_WIDTH as f32;
    let y_sidebar = resolution.1 % SCREEN_HEIGHT as f32;
    let x = (point.0 - x_sidebar / 2.0) * (SCREEN_WIDTH as f32 / (resolution.0 - x_sidebar));
    let y = (point.1 - y_sidebar / 2.0) * (SCREEN_HEIGHT as f32 / (resolution.1 - y_sidebar));
    (x as u32, y as u32)
}
