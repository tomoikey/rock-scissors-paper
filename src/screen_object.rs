use std::ops::{Deref, DerefMut};

use crate::object::Object;
use crate::paper::Paper;
use crate::rock::Rock;
use crate::scissors::Scissors;

#[derive(Debug, Clone, Copy)]
pub enum ScreenObject {
    Paper(Paper),
    Rock(Rock),
    Scissors(Scissors),
}

impl ScreenObject {
    fn battle(&self, other: &ScreenObject) -> Option<bool> {
        match (self, other) {
            (ScreenObject::Paper(_), ScreenObject::Rock(_)) => Some(true),
            (ScreenObject::Rock(_), ScreenObject::Scissors(_)) => Some(true),
            (ScreenObject::Scissors(_), ScreenObject::Paper(_)) => Some(true),
            (ScreenObject::Paper(_), ScreenObject::Scissors(_)) => Some(false),
            (ScreenObject::Rock(_), ScreenObject::Paper(_)) => Some(false),
            (ScreenObject::Scissors(_), ScreenObject::Rock(_)) => Some(false),
            _ => None,
        }
    }

    pub fn collide(&mut self, other: &mut ScreenObject) {
        self.deref_mut().collide(other.deref_mut());

        match (&self, self.battle(other)) {
            (ScreenObject::Paper(_), Some(true)) => {
                let paper = Paper::from(**other);
                *other = ScreenObject::Paper(paper);
            }
            (ScreenObject::Rock(_), Some(true)) => {
                let rock = Rock::from(**other);
                *other = ScreenObject::Rock(rock);
            }
            (ScreenObject::Scissors(_), Some(true)) => {
                let scissors = Scissors::from(**other);
                *other = ScreenObject::Scissors(scissors);
            }
            (ScreenObject::Paper(_), Some(false)) => {
                let scissors = Scissors::from(**self);
                *self = ScreenObject::Scissors(scissors);
            }
            (ScreenObject::Rock(_), Some(false)) => {
                let paper = Paper::from(**self);
                *self = ScreenObject::Paper(paper);
            }
            (ScreenObject::Scissors(_), Some(false)) => {
                let rock = Rock::from(**self);
                *self = ScreenObject::Rock(rock);
            }
            _ => {}
        }
    }
}

impl Deref for ScreenObject {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        match self {
            ScreenObject::Paper(paper) => paper,
            ScreenObject::Rock(rock) => rock,
            ScreenObject::Scissors(scissors) => scissors,
        }
    }
}

impl DerefMut for ScreenObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ScreenObject::Paper(paper) => paper,
            ScreenObject::Rock(rock) => rock,
            ScreenObject::Scissors(scissors) => scissors,
        }
    }
}
