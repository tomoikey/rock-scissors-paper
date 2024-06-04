use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::position::Position;
use crate::velocity::Velocity;

#[derive(Debug, Clone, Copy)]
pub struct Object {
    position: Position,
    color: Color,
    width: u32,
    height: u32,
    mass: f64,
    velocity: Velocity,
}

impl Object {
    pub fn new(
        position: Position,
        color: Color,
        width: u32,
        height: u32,
        mass: f64,
        velocity: Velocity,
    ) -> Object {
        assert!(mass > 0f64, "mass must be greater than 0");
        Object {
            position,
            color,
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
    pub fn color(&self) -> Color {
        self.color
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
    pub fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.mass * self.velocity.velocity().powi(2)
    }

    pub fn collide(&mut self, other: &Object) {
        // 運動エネルギーの総量
        let self_velocity_x = ((self.mass - other.mass) * self.velocity.x()
            + 2.0 * other.mass * other.velocity.x())
            / (self.mass + other.mass);
        let self_velocity_y = ((self.mass - other.mass) * self.velocity.y()
            + 2.0 * other.mass * other.velocity.y())
            / (self.mass + other.mass);
        self.velocity = Velocity::new(self_velocity_x, self_velocity_y);
    }

    pub fn move_right(&mut self, distance: i32) {
        self.position.move_right(distance);
    }

    pub fn move_left(&mut self, distance: i32) {
        self.position.move_left(distance);
    }

    pub fn move_down(&mut self, distance: i32) {
        self.position.move_down(distance);
    }

    pub fn move_up(&mut self, distance: i32) {
        self.position.move_up(distance);
    }

    pub fn move_naturally(&mut self) {
        if self.hit_wall_x() {
            self.velocity = Velocity::new(-self.velocity.x(), self.velocity.y());
        }
        if self.hit_wall_y() {
            self.velocity = Velocity::new(self.velocity.x(), -self.velocity.y());
        }
        self.position.move_right(self.velocity.x() as i32);
        self.position.move_down(self.velocity.y() as i32);
    }

    pub fn hit_wall_x(&self) -> bool {
        self.position.x() <= 0 || self.position.x() >= SCREEN_WIDTH as i32 - self.width as i32
    }

    pub fn hit_wall_y(&self) -> bool {
        self.position.y() <= 0 || self.position.y() >= SCREEN_HEIGHT as i32 - self.height as i32
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) {
        canvas.set_draw_color(color);
        canvas
            .fill_rect(Rect::new(
                self.position.x(),
                self.position.y(),
                self.width,
                self.height,
            ))
            .unwrap();
    }
}
