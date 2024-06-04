#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    x: f64,
    y: f64,
}

impl Velocity {
    pub fn new(x: f64, y: f64) -> Velocity {
        Velocity { x, y }
    }

    pub fn random(max_x: f64, max_y: f64) -> Velocity {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self::new(rng.gen_range(0.5..max_x), rng.gen_range(0.5..max_y))
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn velocity(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn reverse_x(&mut self) {
        self.x = -self.x;
    }

    pub fn reverse_y(&mut self) {
        self.y = -self.y;
    }
}
