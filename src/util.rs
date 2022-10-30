use crate::resources::Level;
use engine::types::{VirtualKeyCode, WinitInputHelper};

pub fn any_key_held(input: &WinitInputHelper, keys: &[VirtualKeyCode]) -> bool {
    for key in keys {
        if input.key_held(*key) {
            return true;
        }
    }
    false
}

pub fn get_tile(x: i32, y: i32, level: &Level) -> char {
    if x >= 0 && x < level.width as i32 && y >= 0 && y < level.height as i32 {
        level
            .repr
            .chars()
            .nth((y * level.width as i32 + x) as usize)
            .unwrap()
    } else {
        ' '
    }
}
