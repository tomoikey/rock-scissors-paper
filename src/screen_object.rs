use log::debug;

use crate::object::Object;
use crate::paper::Paper;
use crate::rock::Rock;
use crate::scissors::Scissors;

#[derive(Clone, Copy)]
pub enum ScreenObject<'a, 'r> {
    Paper(Paper<'a, 'r>),
    Rock(Rock<'a, 'r>),
    Scissors(Scissors<'a, 'r>),
}

impl<'a, 'r> ScreenObject<'a, 'r> {
    fn battle(&mut self, other: &mut ScreenObject) {
        match (&self, &other) {
            (ScreenObject::Paper(_), ScreenObject::Rock(rock)) => {
                debug!("Paper wins Rock");
                *other = ScreenObject::Paper(Paper::from(*rock));
            }
            (ScreenObject::Rock(_), ScreenObject::Scissors(scissors)) => {
                debug!("Rock wins Scissors");
                *other = ScreenObject::Rock(Rock::from(*scissors));
            }
            (ScreenObject::Scissors(_), ScreenObject::Paper(paper)) => {
                debug!("Scissors wins Paper");
                *other = ScreenObject::Scissors(Scissors::from(*paper));
            }
            (ScreenObject::Paper(paper), ScreenObject::Scissors(_)) => {
                debug!("Paper loses Scissors");
                *self = ScreenObject::Scissors(Scissors::from(*paper));
            }
            (ScreenObject::Rock(rock), ScreenObject::Paper(_)) => {
                debug!("Rock loses Paper");
                *self = ScreenObject::Paper(Paper::from(*rock));
            }
            (ScreenObject::Scissors(scissors), ScreenObject::Rock(_)) => {
                debug!("Scissors loses Rock");
                *self = ScreenObject::Rock(Rock::from(*scissors));
            }
            _ => {}
        }
    }

    pub fn object(&self) -> Object {
        match self {
            ScreenObject::Paper(paper) => *paper.object(),
            ScreenObject::Rock(rock) => *rock.object(),
            ScreenObject::Scissors(scissors) => *scissors.object(),
        }
    }

    pub fn object_mut(&mut self) -> &mut Object {
        match self {
            ScreenObject::Paper(paper) => paper.object_mut(),
            ScreenObject::Rock(rock) => rock.object_mut(),
            ScreenObject::Scissors(scissors) => scissors.object_mut(),
        }
    }

    pub fn collide(&mut self, other: &mut ScreenObject<'a, 'r>) {
        self.battle(other);
        self.object_mut().collide(other.object_mut());
    }
}
