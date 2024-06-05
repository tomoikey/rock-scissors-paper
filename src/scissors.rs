use std::ops::{Deref, DerefMut};

use sdl2::pixels::Color;

use crate::object::Object;
use crate::position::Position;
use crate::velocity::Velocity;

#[derive(Debug, Clone, Copy)]
pub struct Scissors(Object);

impl Scissors {
    pub fn new(position: Position, width: u32, height: u32, mass: f64, velocity: Velocity) -> Self {
        assert!(mass > 0f64, "mass must be greater than 0");
        let object = Object::new(
            position,
            Color::RGB(0, 255, 0),
            width,
            height,
            mass,
            velocity,
        );
        Self(object)
    }
}

impl Deref for Scissors {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Scissors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
