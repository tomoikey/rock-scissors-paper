use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use uuid::Uuid;

use crate::screen_object::ScreenObject;

pub struct Screen<'a, 'r, 'ttf_module, 'rwops> {
    width: u32,
    height: u32,
    objects: HashMap<Uuid, RefCell<ScreenObject<'a, 'r>>>,
    font: Font<'ttf_module, 'rwops>,
}

impl<'a, 'r, 'ttf_module, 'rwops> Screen<'a, 'r, 'ttf_module, 'rwops> {
    pub fn new(
        width: u32,
        height: u32,
        font: Font<'ttf_module, 'rwops>,
    ) -> Screen<'a, 'r, 'ttf_module, 'rwops> {
        Screen {
            width,
            height,
            objects: HashMap::new(),
            font,
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
                }
                Collision::TopWall | Collision::BottomWall => {
                    let object = self.objects.get_mut(id).unwrap().get_mut().object_mut();
                    object.velocity_mut().reverse_y();
                    object.next_frame();
                }
                Collision::Object(other_id) => {
                    let (mut self_object, mut other_object) = (
                        self.objects.get(id).unwrap().borrow_mut(),
                        self.objects.get(other_id).unwrap().borrow_mut(),
                    );
                    self_object.collide(&mut other_object);
                    self_object.object_mut().next_frame();
                    other_object.object_mut().next_frame();
                }
            }
        }
    }

    fn draw_text(&mut self, canvas: &mut Canvas<Window>, text: &str) {
        let surface = self
            .font
            .render(text)
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())
            .unwrap();
        let texture_creator = canvas.texture_creator();
        let font_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        let font_width = surface.width();
        let font_height = surface.height();
        canvas
            .copy(
                &font_texture,
                None,
                Rect::new(0, 0, font_width, font_height),
            )
            .unwrap();
    }

    fn object_count(&self) -> (usize, usize, usize) {
        let (mut rock_count, mut paper_count, mut scissors_count) = (0, 0, 0);
        for object in self.objects.values() {
            match object.borrow().deref() {
                ScreenObject::Paper(_) => paper_count += 1,
                ScreenObject::Rock(_) => rock_count += 1,
                ScreenObject::Scissors(_) => scissors_count += 1,
            }
        }
        (rock_count, paper_count, scissors_count)
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.draw_text(
            canvas,
            format!(
                "Rock: {}, Paper: {}, Scissors: {}",
                self.object_count().0,
                self.object_count().1,
                self.object_count().2
            )
            .as_str(),
        );
        self.next_frame();
        for object in self.objects.values_mut() {
            match object.borrow().deref() {
                ScreenObject::Paper(paper) => paper.draw(canvas),
                ScreenObject::Rock(rock) => rock.draw(canvas),
                ScreenObject::Scissors(scissors) => scissors.draw(canvas),
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
