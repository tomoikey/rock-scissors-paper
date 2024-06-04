#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    x: f64,
    y: f64,
}

impl Velocity {
    pub fn new(x: f64, y: f64) -> Velocity {
        Velocity { x, y }
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
}
