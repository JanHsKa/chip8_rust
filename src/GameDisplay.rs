use crate::sdl2;
use crate::constants;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use sdl2::render;
use sdl2::render::Texture;
use std::mem::size_of;

use constants::ROWS;
use constants::COLUMNS;
use constants::SCALE;

pub struct GameDisplay {
    canvas: render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl GameDisplay {

    pub fn new() -> GameDisplay {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let sdl_window = video.window("Chip 8", (SCALE * COLUMNS) as u32 , (SCALE * ROWS) as u32)
            .position_centered()
            .build()
            .expect("Error: Could not init Window");

        let canvas = sdl_window.into_canvas().build()
            .expect("could not init canvas");

        let event_pump = sdl_context.event_pump().unwrap();

        GameDisplay {
            canvas: canvas,
            event_pump: event_pump,
        }
    }

    pub fn initialize(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn draw(&mut self, pixels: &[u8; COLUMNS * ROWS ]) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::ARGB8888,
            COLUMNS as u32,
            ROWS as u32)
            .unwrap();

        texture.update(None, pixels, COLUMNS * size_of::<u32>());
        self.canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.clear();
            texture_canvas.present();
        });
    }

    pub fn check_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                _ => {}
            }
        }
    }
}