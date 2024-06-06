use std::env;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::paper::Paper;
use crate::position::Position;
use crate::rock::Rock;
use crate::scissors::Scissors;
use crate::screen::Screen;
use crate::screen_object::ScreenObject;
use crate::velocity::Velocity;

mod object;
mod paper;
mod position;
mod rock;
mod scissors;
mod screen;
mod screen_object;
mod velocity;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 800;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rock Scissors Paper Battle", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();

    // init logger
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut screen = Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let rock_texture = texture_creator.load_texture("assets/rock.png").unwrap();
    let paper_texture = texture_creator.load_texture("assets/paper.png").unwrap();
    let scissors_texture = texture_creator.load_texture("assets/scissors.png").unwrap();

    let objects_count = 2;
    for i in 0..objects_count {
        let size = 200 / objects_count;
        let (position, width, height, mass, velocity) = (
            Position::random(
                (SCREEN_WIDTH - size) as f64,
                (SCREEN_HEIGHT - size) as f64,
                (SCREEN_WIDTH - size) as f64,
                (SCREEN_HEIGHT - size) as f64,
            ),
            size,
            size,
            1f64,
            Velocity::random(1.3, 1.3),
        );

        let object = match i % 3 {
            0 => ScreenObject::Paper(Paper::new(
                position,
                width,
                height,
                mass,
                velocity,
                &paper_texture,
                &scissors_texture,
                &rock_texture,
            )),
            1 => ScreenObject::Rock(Rock::new(
                position,
                width,
                height,
                mass,
                velocity,
                &rock_texture,
                &paper_texture,
                &scissors_texture,
            )),
            2 => ScreenObject::Scissors(Scissors::new(
                position,
                width,
                height,
                mass,
                velocity,
                &scissors_texture,
                &rock_texture,
                &paper_texture,
            )),
            _ => unreachable!(),
        };

        screen.add_object(object);
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { .. } => {}
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        screen.draw(&mut canvas);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
    }
}
