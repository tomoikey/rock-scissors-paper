use sdl2::render::Texture;

use crate::object::Object;
use crate::paper::Paper;
use crate::position::Position;
use crate::velocity::Velocity;

#[derive(Clone, Copy)]
pub struct Scissors<'a, 'r> {
    object: Object,
    texture: &'a Texture<'r>,
    rock_texture: &'a Texture<'r>,
    paper_texture: &'a Texture<'r>,
}

impl<'a, 'r> Scissors<'a, 'r> {
    pub fn new(
        position: Position,
        width: u32,
        height: u32,
        mass: f64,
        velocity: Velocity,
        texture: &'a Texture<'r>,
        rock_texture: &'a Texture<'r>,
        paper_texture: &'a Texture<'r>,
    ) -> Self {
        assert!(mass > 0f64, "mass must be greater than 0");
        let object = Object::new(position, width, height, mass, velocity);
        Self {
            object,
            texture,
            rock_texture,
            paper_texture,
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
    pub fn rock_texture(&self) -> &'a Texture<'r> {
        self.rock_texture
    }

    #[inline]
    pub fn paper_texture(&self) -> &'a Texture<'r> {
        self.paper_texture
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.object.draw(canvas, self.texture);
    }
}

impl<'a, 'r> From<Paper<'a, 'r>> for Scissors<'a, 'r> {
    fn from(paper: Paper<'a, 'r>) -> Self {
        Self {
            object: *paper.object(),
            texture: paper.scissors_texture(),
            rock_texture: paper.rock_texture(),
            paper_texture: paper.texture(),
        }
    }
}
