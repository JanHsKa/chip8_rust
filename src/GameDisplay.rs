use crate::sdl2;
use crate::constants;
use crate::keypad::Keypad;
use crate::layout_constants;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render;
use sdl2::rect;
use std::rc::Rc;
use std::cell::RefCell;
use sdl2::Sdl;

use constants::ROWS;
use constants::COLUMNS;
use constants::SCALE;

use layout_constants::WINDOW_HEIGHT;
use layout_constants::WINDOW_WIDTH;
use layout_constants::GAME_START_X;
use layout_constants::GAME_START_Y;
use layout_constants::MEMORY_HEIGHT;
use layout_constants::MEMORY_WIDTH;
use layout_constants::CONTROL_START_X;
use layout_constants::CONTROL_START_Y;
use layout_constants::OUTLINE;


pub struct GameDisplay {
    canvas: render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    keypad:  Rc<RefCell<Keypad>>,
    quit: bool,
}

impl GameDisplay {
    pub fn new(new_keypad: Rc<RefCell<Keypad>>, context: &Sdl) -> GameDisplay {
        let video = context.video().unwrap();
        let sdl_window = video.window("Chip 8", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .expect("Error: Could not init Window");

        let canvas = sdl_window.into_canvas().build()
            .expect("could not init canvas");

        let event_pump = context.event_pump().unwrap();

        GameDisplay {
            canvas: canvas,
            event_pump: event_pump,
            keypad: new_keypad,
            quit: false,
        }
    }

    pub fn initialize(&mut self) {
        self.canvas.set_draw_color(layout_constants::WINDOW_BACKGROUND);
        let mut rect = rect::Rect::new(0, 0 , WINDOW_WIDTH, WINDOW_HEIGHT); 
        self.canvas.fill_rect(rect);

        //HORIZONTAL
        self.canvas.set_draw_color(layout_constants::DARK_OUTLINE);
        rect.set_x(layout_constants::EDGE_SIZE + OUTLINE);
        rect.set_y(layout_constants::EDGE_SIZE);
        rect.set_width(layout_constants::GAME_WIDTH);
        rect.set_height(OUTLINE as u32);
        self.canvas.fill_rect(rect);

        rect.set_x(layout_constants::EDGE_SIZE + OUTLINE);
        rect.set_y(layout_constants::EDGE_SIZE);
        rect.set_width(layout_constants::GAME_WIDTH);
        self.canvas.fill_rect(rect);

        //VERTIKAL
        rect.set_x(layout_constants::EDGE_SIZE);
        rect.set_y(layout_constants::EDGE_SIZE);
        rect.set_width(OUTLINE as u32);
        rect.set_height(layout_constants::GAME_HEIGHT + OUTLINE as u32);
        self.canvas.fill_rect(rect);

        self.canvas.set_draw_color(layout_constants::BRIGHT_OUTLINE);

        //HORIZONTAL
        rect.set_x(layout_constants::EDGE_SIZE);
        rect.set_y(layout_constants::EDGE_SIZE + layout_constants::GAME_HEIGHT as i32+ OUTLINE);
        rect.set_width(layout_constants::GAME_WIDTH + OUTLINE as u32);
        rect.set_height(OUTLINE as u32);
        self.canvas.fill_rect(rect);

        //VERTIKAL
        rect.set_x(GAME_START_X + layout_constants::GAME_WIDTH as i32);
        rect.set_y(layout_constants::EDGE_SIZE);
        rect.set_width(OUTLINE as u32);
        rect.set_height(layout_constants::GAME_HEIGHT + 2 * OUTLINE as u32);
        self.canvas.fill_rect(rect);

        /* rect.set_x(CONTROL_START_X);
        rect.set_y(CONTROL_START_Y);
        rect.set_width(MEMORY_WIDTH);
        rect.set_height(MEMORY_HEIGHT);
        self.canvas.fill_rect(rect); */

        self.canvas.present();
    }

    pub fn draw(&mut self, pixels: [u8; COLUMNS * ROWS ]) {
        /* let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::ARGB8888,
            COLUMNS as u32,
            ROWS as u32)
            .expect("Error: Could not create texture"); */

        //texture.update(None, &pixels, COLUMNS * 4).expect("Error: Could not copy framebuffer to texture");

        // self.canvas.copy(&texture, None, None).expect("Error: Could not copy texture to canvas");
        self.draw_pixels(pixels);
        self.canvas.present();
    }

    fn draw_pixels(&mut self, pixels: [u8; COLUMNS * ROWS]) {
        let mut rect = rect::Rect::new(GAME_START_X, GAME_START_Y , SCALE as u32, SCALE as u32); 
        for y in 0..ROWS {
            rect.set_y((y * SCALE) as i32 + GAME_START_Y);
            for x in 0..COLUMNS {
                if pixels[(y * COLUMNS) + x] == 1 {
                    self.canvas.set_draw_color(Color::RGB(170, 255, 170));
                } else {
                    self.canvas.set_draw_color(Color::RGB(40, 40, 40));
                }
                rect.set_x((x * SCALE) as i32 + GAME_START_X);
                self.canvas.fill_rect(rect).expect("Error: could not draw pixel");
            }
        }
    }

    pub fn get_quit(&mut self) -> bool {
        self.quit
    }

    pub fn check_input(&mut self) {
        let mut keypad_ref = self.keypad.borrow_mut();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown {keycode,..} => (*keypad_ref).press_key(keycode.unwrap(), 1),
                Event::KeyUp {keycode,..} => (*keypad_ref).press_key(keycode.unwrap(), 0),
                Event::Quit {..} => { self.quit = true },
                _ => {}
            }
        }
    }
}