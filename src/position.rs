#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: f64,
    y: f64,
    max_x: f64,
    max_y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, max_x: f64, max_y: f64) -> Position {
        Position { x, y, max_x, max_y }
    }

    pub fn random(max_width: f64, max_height: f64, max_x: f64, max_y: f64) -> Position {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(0.1..max_width),
            rng.gen_range(0.1..max_height),
            max_x,
            max_y,
        )
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn move_distance(&mut self, x: f64, y: f64) {
        let new_x = self.x + x;
        let new_y = self.y + y;
        if new_x <= self.max_x && new_x >= 0.0 {
            self.x = new_x;
        } else {
            self.x = if new_x > self.max_x { self.max_x } else { 0.0 };
        }
        if new_y <= self.max_y && new_y >= 0.0 {
            self.y = new_y;
        } else {
            self.y = if new_y > self.max_y { self.max_y } else { 0.0 };
        }
    }
}
