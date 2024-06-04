use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::position::Position;

#[derive(Debug, Clone, Copy)]
pub struct Object {
    position: Position,
    color: Color,
    width: u32,
    height: u32,
}

impl Object {
    pub fn new(position: Position, color: Color, width: u32, height: u32) -> Object {
        Object {
            position,
            color,
            width,
            height,
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
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

    pub fn hit_wall(&self, screen_width: u32, screen_height: u32) -> bool {
        self.position.x() == 0
            || self.position.x() == screen_width as i32 - self.width as i32
            || self.position.y() == 0
            || self.position.y() == screen_height as i32 - self.height as i32
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
