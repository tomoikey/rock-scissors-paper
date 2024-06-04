use std::collections::HashMap;

use log::debug;
use sdl2::render::Canvas;
use sdl2::video::Window;
use uuid::Uuid;

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Screen {
    width: u32,
    height: u32,
    objects: HashMap<Uuid, Object>,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        Screen {
            width,
            height,
            objects: HashMap::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) -> Uuid {
        let id = Uuid::new_v4();
        self.objects.insert(id, object);
        id
    }

    pub fn get_object_mut(&mut self, object_id: Uuid) -> Option<&mut Object> {
        self.objects.get_mut(&object_id)
    }

    pub fn next_frame(&mut self) {
        let mut collisions = HashMap::new();

        let mut checked_objects = HashMap::<Uuid, &Object>::new();
        for (id, object) in self.objects.iter() {
            let mut collision = Collision::None;

            // 壁との衝突判定
            if object.position().x() <= 0.0 {
                collision = Collision::LeftWall;
            } else if object.position().x() + object.width() as f64 >= self.width as f64 {
                collision = Collision::RightWall;
            } else if object.position().y() <= 0.0 {
                collision = Collision::TopWall;
            } else if object.position().y() + object.height() as f64 >= self.height as f64 {
                collision = Collision::BottomWall;
            }

            // 他のオブジェクトとの衝突判定
            for (checked_other_id, checked_other_object) in checked_objects.iter() {
                if object.is_collide_with(checked_other_object) {
                    collision = Collision::Object(*checked_other_id);
                }
            }

            checked_objects.insert(*id, object);
            collisions.insert(*id, collision);
        }

        for (id, collision) in collisions.iter() {
            match collision {
                Collision::None => {
                    let object = self.objects.get_mut(id).unwrap();
                    object.next_frame();
                }
                Collision::LeftWall | Collision::RightWall => {
                    let object = self.objects.get_mut(id).unwrap();
                    object.velocity_mut().reverse_x();
                    object.next_frame();

                    debug!("Object({:?}) collided with wall", id);
                }
                Collision::TopWall | Collision::BottomWall => {
                    let object = self.objects.get_mut(id).unwrap();
                    object.velocity_mut().reverse_y();
                    object.next_frame();

                    debug!("Object({:?}) collided with wall", id);
                }
                Collision::Object(other_id) => {
                    let self_object = self.objects.get(id).unwrap();
                    let other_object = self.objects.get(other_id).unwrap();

                    let (mut new_self_object, mut new_other_object) =
                        self_object.collide(other_object);

                    new_self_object.next_frame();
                    new_other_object.next_frame();
                    self.objects.insert(*id, new_self_object);
                    self.objects.insert(*other_id, new_other_object);

                    debug!("Object({:?}) collided with Object({:?})", id, other_id);
                }
            }
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.next_frame();
        for object in self.objects.values_mut() {
            object.draw(canvas, object.color());
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
