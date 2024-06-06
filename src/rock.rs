use sdl2::render::Texture;

use crate::object::Object;
use crate::position::Position;
use crate::scissors::Scissors;
use crate::velocity::Velocity;

#[derive(Clone, Copy)]
pub struct Rock<'a, 'r> {
    object: Object,
    texture: &'a Texture<'r>,
    paper_texture: &'a Texture<'r>,
    scissors_texture: &'a Texture<'r>,
}

impl<'a, 'r> Rock<'a, 'r> {
    pub fn new(
        position: Position,
        width: u32,
        height: u32,
        mass: f64,
        velocity: Velocity,
        texture: &'a Texture<'r>,
        paper_texture: &'a Texture<'r>,
        scissors_texture: &'a Texture<'r>,
    ) -> Self {
        assert!(mass > 0f64, "mass must be greater than 0");
        let object = Object::new(position, width, height, mass, velocity);
        Self {
            object,
            texture,
            paper_texture,
            scissors_texture,
        }
    }

    pub fn object(&self) -> &Object {
        &self.object
    }

    pub fn object_mut(&mut self) -> &mut Object {
        &mut self.object
    }

    #[inline]
    pub fn texture(&self) -> &'a Texture<'r> {
        self.texture
    }

    #[inline]
    pub fn paper_texture(&self) -> &'a Texture<'r> {
        self.paper_texture
    }

    #[inline]
    pub fn scissors_texture(&self) -> &'a Texture<'r> {
        self.scissors_texture
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.object.draw(canvas, self.texture);
    }
}

impl<'a, 'r> From<Scissors<'a, 'r>> for Rock<'a, 'r> {
    fn from(scissors: Scissors<'a, 'r>) -> Self {
        Self {
            object: *scissors.object(),
            texture: scissors.rock_texture(),
            paper_texture: scissors.paper_texture(),
            scissors_texture: scissors.texture(),
        }
    }
}
