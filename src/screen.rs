use std::collections::HashMap;

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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn add_object(&mut self, object: Object) -> Uuid {
        let id = Uuid::new_v4();
        self.objects.insert(id, object);
        id
    }

    pub fn get_object(&self, object_id: Uuid) -> Option<&Object> {
        self.objects.get(&object_id)
    }

    pub fn get_object_mut(&mut self, object_id: Uuid) -> Option<&mut Object> {
        for (id, object) in self.objects.iter_mut() {
            if *id == object_id {
                return Some(object);
            }
        }
        None
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        for object in self.objects.values_mut() {
            object.move_naturally();
            object.draw(canvas, object.color());
        }
    }
}
