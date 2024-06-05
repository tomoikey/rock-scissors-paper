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
