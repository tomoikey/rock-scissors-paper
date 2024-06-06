use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::position::Position;
use crate::velocity::Velocity;

#[derive(Debug, Clone, Copy)]
pub struct Object {
    position: Position,
    width: u32,
    height: u32,
    mass: f64,
    velocity: Velocity,
}

impl Object {
    pub fn new(
        position: Position,
        width: u32,
        height: u32,
        mass: f64,
        velocity: Velocity,
    ) -> Object {
        assert!(mass > 0f64, "mass must be greater than 0");
        Object {
            position,
            width,
            height,
            mass,
            velocity,
        }
    }

    #[inline]
    pub fn position(&self) -> Position {
        self.position
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn mass(&self) -> f64 {
        self.mass
    }

    #[inline]
    pub fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn range_x(&self) -> (f64, f64) {
        (self.position.x(), self.position.x() + self.width as f64)
    }

    pub fn range_y(&self) -> (f64, f64) {
        (self.position.y(), self.position.y() + self.height as f64)
    }

    pub fn is_collide_with(&self, other: &Object) -> bool {
        // selfの左の辺がotherの右の辺と重なっているか
        let is_left_collide = (self.range_x().0 >= other.range_x().0)
            && (self.range_x().0 <= other.range_x().1)
            && ((self.range_y().0 >= other.range_y().0) && (self.range_y().0 <= other.range_y().1)
                || (self.range_y().1 >= other.range_y().0)
                    && (self.range_y().1 <= other.range_y().1));

        // selfの上の辺がotherの下の辺と重なっているか
        let is_top_collide = (self.range_y().0 >= other.range_y().0)
            && (self.range_y().0 <= other.range_y().1)
            && ((self.range_x().0 >= other.range_x().0) && (self.range_x().0 <= other.range_x().1)
                || (self.range_x().1 >= other.range_x().0)
                    && (self.range_x().1 <= other.range_x().1));

        // selfの右の辺がotherの左の辺と重なっているか
        let is_right_collide = (self.range_x().1 >= other.range_x().0)
            && (self.range_x().1 <= other.range_x().1)
            && ((self.range_y().0 >= other.range_y().0) && (self.range_y().0 <= other.range_y().1)
                || (self.range_y().1 >= other.range_y().0)
                    && (self.range_y().1 <= other.range_y().1));

        // selfの下の辺がotherの上の辺と重なっているか
        let is_bottom_collide = (self.range_y().1 >= other.range_y().0)
            && (self.range_y().1 <= other.range_y().1)
            && ((self.range_x().0 >= other.range_x().0) && (self.range_x().0 <= other.range_x().1)
                || (self.range_x().1 >= other.range_x().0)
                    && (self.range_x().1 <= other.range_x().1));

        is_left_collide || is_top_collide || is_right_collide || is_bottom_collide
    }

    pub fn collide(&mut self, other: &mut Object) {
        let self_velocity_x = ((self.mass - other.mass) * self.velocity.x()
            + 2.0 * other.mass * other.velocity.x())
            / (self.mass + other.mass);
        let self_velocity_y = ((self.mass - other.mass) * self.velocity.y()
            + 2.0 * other.mass * other.velocity.y())
            / (self.mass + other.mass);

        let other_velocity_x = ((other.mass - self.mass) * other.velocity.x()
            + 2.0 * self.mass * self.velocity.x())
            / (self.mass + other.mass);
        let other_velocity_y = ((other.mass - self.mass) * other.velocity.y()
            + 2.0 * self.mass * self.velocity.y())
            / (self.mass + other.mass);

        self.velocity = Velocity::new(self_velocity_x, self_velocity_y);
        other.velocity = Velocity::new(other_velocity_x, other_velocity_y);
    }

    pub fn next_frame(&mut self) {
        self.position
            .move_distance(self.velocity.x(), self.velocity.y());
    }

    pub fn velocity_mut(&mut self) -> &mut Velocity {
        &mut self.velocity
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, texture: &Texture) {
        canvas
            .copy(
                texture,
                None,
                Some(Rect::new(
                    self.position.x() as i32,
                    self.position.y() as i32,
                    self.width,
                    self.height,
                )),
            )
            .expect("failed to copy texture");
    }
}
