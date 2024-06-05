use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;

use log::debug;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use uuid::Uuid;

use crate::screen_object::ScreenObject;

#[derive(Clone)]
pub struct Screen<'a, 'r> {
    width: u32,
    height: u32,
    objects: HashMap<Uuid, RefCell<ScreenObject<'a, 'r>>>,
}

impl<'a, 'r> Screen<'a, 'r> {
    pub fn new(width: u32, height: u32) -> Screen<'a, 'r> {
        Screen {
            width,
            height,
            objects: HashMap::new(),
        }
    }

    pub fn add_object(&mut self, object: ScreenObject<'a, 'r>) -> Uuid {
        let id = Uuid::new_v4();
        self.objects.insert(id, RefCell::new(object));
        id
    }

    pub fn next_frame(&mut self) {
        let mut collisions = HashMap::new();

        let mut checked_objects = HashMap::<Uuid, ScreenObject>::new();
        for (id, object) in self.objects.iter() {
            let object = object.borrow();
            let mut collision = Collision::None;

            // 壁との衝突判定
            if object.object().position().x() <= 0.0 {
                collision = Collision::LeftWall;
            } else if object.object().position().x() + object.object().width() as f64
                >= self.width as f64
            {
                collision = Collision::RightWall;
            } else if object.object().position().y() <= 0.0 {
                collision = Collision::TopWall;
            } else if object.object().position().y() + object.object().height() as f64
                >= self.height as f64
            {
                collision = Collision::BottomWall;
            }

            // 他のオブジェクトとの衝突判定
            for (checked_other_id, checked_other_object) in checked_objects.iter() {
                if object
                    .object()
                    .is_collide_with(&checked_other_object.object())
                {
                    collision = Collision::Object(*checked_other_id);
                }
            }

            checked_objects.insert(*id, *object.deref());
            collisions.insert(*id, collision);
        }

        for (id, collision) in collisions.iter() {
            match collision {
                Collision::None => {
                    let object = self.objects.get_mut(id).unwrap().get_mut().object_mut();
                    object.next_frame();
                }
                Collision::LeftWall | Collision::RightWall => {
                    let object = self.objects.get_mut(id).unwrap().get_mut().object_mut();
                    object.velocity_mut().reverse_x();
                    object.next_frame();

                    debug!("Object({:?}) collided with wall", id);
                }
                Collision::TopWall | Collision::BottomWall => {
                    let object = self.objects.get_mut(id).unwrap().get_mut().object_mut();
                    object.velocity_mut().reverse_y();
                    object.next_frame();

                    debug!("Object({:?}) collided with wall", id);
                }
                Collision::Object(other_id) => {
                    let (mut self_object, mut other_object) = (
                        self.objects.get(id).unwrap().borrow_mut(),
                        self.objects.get(other_id).unwrap().borrow_mut(),
                    );
                    self_object.object_mut().collide(other_object.object_mut());
                    self_object.object_mut().next_frame();
                    other_object.object_mut().next_frame();

                    debug!("Object({:?}) collided with Object({:?})", id, other_id);
                }
            }
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.next_frame();
        for object in self.objects.values_mut() {
            match object.borrow().deref() {
                ScreenObject::Paper(paper, _) => paper.draw(canvas),
                ScreenObject::Rock(rock, _) => rock.draw(canvas),
                ScreenObject::Scissors(scissors, _) => scissors.draw(canvas),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Collision {
    None,
    TopWall,
    BottomWall,
    LeftWall,
    RightWall,
    Object(Uuid),
}
