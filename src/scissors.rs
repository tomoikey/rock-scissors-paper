use sdl2::render::Texture;

use crate::object::Object;
use crate::position::Position;
use crate::velocity::Velocity;

#[derive(Clone, Copy)]
pub struct Scissors<'a, 'r> {
    object: Object,
    texture: &'a Texture<'r>,
}

impl<'a, 'r> Scissors<'a, 'r> {
    pub fn new(
        position: Position,
        width: u32,
        height: u32,
        mass: f64,
        velocity: Velocity,
        texture: &'a Texture<'r>,
    ) -> Self {
        assert!(mass > 0f64, "mass must be greater than 0");
        let object = Object::new(position, width, height, mass, velocity);
        Self { object, texture }
    }

    pub fn object(&self) -> &Object {
        &self.object
    }

    pub fn object_mut(&mut self) -> &mut Object {
        &mut self.object
    }

    pub fn from_object(object: Object, texture: &'a Texture<'r>) -> Self {
        let object = Object::new(
            object.position(),
            object.width(),
            object.height(),
            object.mass(),
            object.velocity(),
        );
        Self { object, texture }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.object.draw(canvas, self.texture);
    }
}
