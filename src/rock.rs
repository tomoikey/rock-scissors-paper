use std::ops::{Deref, DerefMut};

use sdl2::pixels::Color;

use crate::object::Object;
use crate::position::Position;
use crate::velocity::Velocity;

#[derive(Debug, Clone, Copy)]
pub struct Rock(Object);

impl Rock {
    pub fn new(position: Position, width: u32, height: u32, mass: f64, velocity: Velocity) -> Self {
        assert!(mass > 0f64, "mass must be greater than 0");
        let object = Object::new(
            position,
            Color::RGB(255, 0, 0),
            width,
            height,
            mass,
            velocity,
        );
        Self(object)
    }
}

impl From<Object> for Rock {
    fn from(object: Object) -> Self {
        let (position, width, height, mass, velocity) = (
            object.position(),
            object.width(),
            object.height(),
            object.mass(),
            object.velocity(),
        );
        Self(Object::new(
            position,
            Color::RGB(255, 0, 0),
            width,
            height,
            mass,
            velocity,
        ))
    }
}

impl Deref for Rock {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
