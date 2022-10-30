use engine::types::VirtualKeyCode;

pub struct Level {
    pub width: u32,
    pub height: u32,
    pub repr: String,
}

impl Level {
    pub fn new_test() -> Self {
        let width = 64;
        let height = 16;
        let mut repr = "".to_owned();
        repr += "################################################################";
        repr += "#...............................................................";
        repr += "#.....................................############........#.....";
        repr += "#................................................#........#.....";
        repr += "#................................................#........#.....";
        repr += "#................................................#........#.....";
        repr += "#.......................#.......................................";
        repr += "#......................##.......................................";
        repr += "#........#............###.###.##...............................#";
        repr += "#....................####.......................................";
        repr += "#...................#####.......................................";
        repr += "####.######################.#######.#.##########################";
        repr += "#.........................#.#.....#.#...........................";
        repr += "#.........................#.###.###.#...........................";
        repr += "#.........................#.........#...........................";
        repr += "#.........................###########...........................";
        Self {
            width,
            height,
            repr,
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
