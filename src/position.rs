#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn move_right(&mut self, distance: i32) {
        self.x += distance;
    }

    pub fn move_left(&mut self, distance: i32) {
        self.x -= distance;
    }

    pub fn move_down(&mut self, distance: i32) {
        self.y += distance;
    }

    pub fn move_up(&mut self, distance: i32) {
        self.y -= distance;
    }
}
