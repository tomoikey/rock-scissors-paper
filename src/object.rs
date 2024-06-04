use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use uuid::Uuid;

use crate::position::Position;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Object {
    id: Uuid,
    position: Position,
    width: u32,
    height: u32,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Object {
    pub fn new(position: Position, width: u32, height: u32) -> Object {
        Object {
            id: Uuid::new_v4(),
            position,
            width,
            height,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn position(&self) -> Position {
        self.position
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
