use engine::types::{VirtualKeyCode, WinitInputHelper};

pub fn any_key_held(input: &WinitInputHelper, keys: &[VirtualKeyCode]) -> bool {
    for key in keys {
        if input.key_held(*key) {
            return true;
        }
    }
    false
}
