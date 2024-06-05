use sdl2::render::Texture;

use crate::object::Object;
use crate::paper::Paper;
use crate::rock::Rock;
use crate::scissors::Scissors;

#[derive(Clone, Copy)]
pub enum ScreenObject<'a, 'r> {
    Paper(Paper<'a, 'r>, &'a Texture<'r>),
    Rock(Rock<'a, 'r>, &'a Texture<'r>),
    Scissors(Scissors<'a, 'r>, &'a Texture<'r>),
}

impl<'a, 'r> ScreenObject<'a, 'r> {
    fn battle(&self, other: &ScreenObject) -> Option<bool> {
        match (self, other) {
            (ScreenObject::Paper(_, _), ScreenObject::Rock(_, _)) => Some(true),
            (ScreenObject::Rock(_, _), ScreenObject::Scissors(_, _)) => Some(true),
            (ScreenObject::Scissors(_, _), ScreenObject::Paper(_, _)) => Some(true),
            (ScreenObject::Paper(_, _), ScreenObject::Scissors(_, _)) => Some(false),
            (ScreenObject::Rock(_, _), ScreenObject::Paper(_, _)) => Some(false),
            (ScreenObject::Scissors(_, _), ScreenObject::Rock(_, _)) => Some(false),
            _ => None,
        }
    }

    pub fn object(&self) -> Object {
        match self {
            ScreenObject::Paper(paper, _) => *paper.object(),
            ScreenObject::Rock(rock, _) => *rock.object(),
            ScreenObject::Scissors(scissors, _) => *scissors.object(),
        }
    }

    pub fn object_mut(&mut self) -> &mut Object {
        match self {
            ScreenObject::Paper(paper, _) => paper.object_mut(),
            ScreenObject::Rock(rock, _) => rock.object_mut(),
            ScreenObject::Scissors(scissors, _) => scissors.object_mut(),
        }
    }

    pub fn collide(&mut self, other: &mut ScreenObject<'a, 'r>) {
        self.object_mut().collide(other.object_mut());

        match (&self, self.battle(other)) {
            (ScreenObject::Paper(_, texture), Some(true)) => {
                let paper = Paper::from_object(other.object(), texture);
                *other = ScreenObject::Paper(paper, texture);
            }
            (ScreenObject::Rock(_, texture), Some(true)) => {
                let rock = Rock::from_object(other.object(), texture);
                *other = ScreenObject::Rock(rock, texture);
            }
            (ScreenObject::Scissors(_, texture), Some(true)) => {
                let scissors = Scissors::from_object(other.object(), texture);
                *other = ScreenObject::Scissors(scissors, texture);
            }
            (ScreenObject::Paper(_, texture), Some(false)) => {
                let scissors = Scissors::from_object(self.object(), texture);
                *self = ScreenObject::Scissors(scissors, texture);
            }
            (ScreenObject::Rock(_, texture), Some(false)) => {
                let paper = Paper::from_object(self.object(), texture);
                *self = ScreenObject::Paper(paper, texture);
            }
            (ScreenObject::Scissors(_, texture), Some(false)) => {
                let rock = Rock::from_object(self.object(), texture);
                *self = ScreenObject::Rock(rock, texture);
            }
            _ => {}
        }
    }
}
