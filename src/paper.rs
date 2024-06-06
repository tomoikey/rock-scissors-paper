use sdl2::render::Texture;

use crate::object::Object;
use crate::position::Position;
use crate::rock::Rock;
use crate::velocity::Velocity;

#[derive(Clone, Copy)]
pub struct Paper<'a, 'r> {
    object: Object,
    texture: &'a Texture<'r>,
    scissors_texture: &'a Texture<'r>,
    rock_texture: &'a Texture<'r>,
}

impl<'a, 'r> Paper<'a, 'r> {
    pub fn new(
        position: Position,
        width: u32,
        height: u32,
        mass: f64,
        velocity: Velocity,
        texture: &'a Texture<'r>,
        scissors_texture: &'a Texture<'r>,
        rock_texture: &'a Texture<'r>,
    ) -> Self {
        assert!(mass > 0f64, "mass must be greater than 0");
        let object = Object::new(position, width, height, mass, velocity);
        Self {
            object,
            texture,
            scissors_texture,
            rock_texture,
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
    pub fn scissors_texture(&self) -> &'a Texture<'r> {
        self.scissors_texture
    }

    #[inline]
    pub fn rock_texture(&self) -> &'a Texture<'r> {
        self.rock_texture
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.object.draw(canvas, self.texture);
    }
}

impl<'a, 'r> From<Rock<'a, 'r>> for Paper<'a, 'r> {
    fn from(rock: Rock<'a, 'r>) -> Self {
        Self {
            object: *rock.object(),
            texture: rock.paper_texture(),
            scissors_texture: rock.scissors_texture(),
            rock_texture: rock.texture(),
        }
    }
}
