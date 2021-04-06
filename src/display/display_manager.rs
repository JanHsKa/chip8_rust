use crate::sdl2;
use crate::processor::memory_constants::{ROWS, COLUMNS};
use crate::display::{layout_constants, MemoryDisplay, InfoDisplay, OpcodeDisplay, GameDisplay};
use crate::utils::{Keypad};
use crate::interfaces::IDisplay;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::{render, Sdl, rect};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::ttf;
use std::rc::Rc;
use std::cell::RefCell;
use sdl2::surface::Surface;
//use sdl2::ttf;
use std::boxed::Box;

pub const FONTPATH1: &str = "Data/Font/PrintChar21.ttf";
pub const FONTPATH2: &str = "Data/Font/8-BIT WONDER.ttf";
pub const FONTPATH3: &str = "Data/Font/C64_Pro-STYLE.ttf";
pub const FONTPATH4: &str = "Data/Font/C64_Pro_Mono-STYLE.ttf";
pub const ICONPATH: &str = "Data/Icons/Chip8_icon_24_21.bmp";

pub const FONTSIZE: u16 = 18;


use self::layout_constants::{PIXEL_SCALE, 
    WINDOW_HEIGHT, WINDOW_WIDTH, GAME_START_X, 
    GAME_START_Y, MEMORY_HEIGHT, MEMORY_WIDTH, 
    INFO_START_X, INFO_START_Y, STACK_HEIGHT,
    STACK_WIDTH, STACK_START_X, STACK_START_Y, 
    OUTLINE, EDGE_SIZE};


pub struct DisplayManager {
    canvas: render::Canvas<sdl2::video::Window>,
    //font: sdl2::ttf::Font;
    keypad:  Rc<RefCell<Keypad>>,
    quit: bool,
    displays: Vec<Box<dyn IDisplay>>,
    ttf_context: Sdl2TtfContext,
}



impl DisplayManager {
    pub fn new(new_keypad: Rc<RefCell<Keypad>>, context: &Sdl) -> DisplayManager {
        let video = context.video().unwrap();
        let mut sdl_window = video.window("Chip 8", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .expect("Error: Could not init Window");

        let window_icon = Surface::load_bmp(ICONPATH).expect("Could not open icon");
        sdl_window.set_icon(window_icon);

        let canvas = sdl_window.into_canvas().build()
            .expect("could not init canvas");

        let ttf = ttf::init().unwrap();

        DisplayManager {
            canvas: canvas,
            keypad: new_keypad,
            quit: false,
            displays: Vec::new(),
            ttf_context: ttf,
        }
    }

    pub fn initialize(&mut self) {
        self.draw_window();

        self.draw_outline();

        self.canvas.present();
    }

    pub fn add_display(&mut self, display: Box<dyn IDisplay>) {
        self.displays.push(display);
    }

    pub fn draw_window(&mut self) {
        self.canvas.set_draw_color(*layout_constants::WINDOW_BACKGROUND);
        let rect = rect::Rect::new(0, 0 , WINDOW_WIDTH, WINDOW_HEIGHT); 
        self.canvas.fill_rect(rect);
    }

    pub fn draw_outline(&mut self) {
        let mut rect = rect::Rect::new(layout_constants::EDGE_SIZE + OUTLINE, 
            layout_constants::EDGE_SIZE, 
            layout_constants::GAME_WIDTH, 
            OUTLINE as u32);

         //HORIZONTAL Dark
         self.canvas.set_draw_color(*layout_constants::DARK_OUTLINE);
         self.canvas.fill_rect(rect);
 
         rect.set_x(layout_constants::OPCODE_START_X);
         rect.set_width(layout_constants::OPCODE_WIDTH);
         self.canvas.fill_rect(rect);
 
         rect.set_x(EDGE_SIZE);
         rect.set_y(layout_constants::INFO_START_Y - OUTLINE);
         rect.set_width(layout_constants::INFO_WIDTH + OUTLINE as u32);
         self.canvas.fill_rect(rect);

         rect.set_x(layout_constants::STACK_START_X - OUTLINE);
         rect.set_y(layout_constants::STACK_START_Y - OUTLINE);
         rect.set_width(layout_constants::STACK_WIDTH + OUTLINE as u32);
         self.canvas.fill_rect(rect);
 
         rect.set_x(layout_constants::MEMORY_START_X - OUTLINE);
         rect.set_y(layout_constants::MEMORY_START_Y - OUTLINE);
         rect.set_width(layout_constants::MEMORY_WIDTH + OUTLINE as u32);
         self.canvas.fill_rect(rect);
 
         //VERTICAL Dark
         rect.set_x(layout_constants::EDGE_SIZE);
         rect.set_y(layout_constants::EDGE_SIZE);
         rect.set_width(OUTLINE as u32);
         rect.set_height(layout_constants::GAME_HEIGHT + OUTLINE as u32);
         self.canvas.fill_rect(rect);
 
         rect.set_x(layout_constants::OPCODE_START_X - OUTLINE);
         self.canvas.fill_rect(rect);
 
         rect.set_x(EDGE_SIZE);
         rect.set_y(layout_constants::INFO_START_Y - OUTLINE);
         rect.set_height(layout_constants::INFO_HEIGHT + OUTLINE as u32);
         self.canvas.fill_rect(rect);

         rect.set_x(layout_constants::STACK_START_X - OUTLINE);
         rect.set_y(layout_constants::STACK_START_Y - OUTLINE);
         rect.set_height(layout_constants::STACK_HEIGHT + OUTLINE as u32);
         self.canvas.fill_rect(rect);
 
         rect.set_x(layout_constants::MEMORY_START_X - OUTLINE);
         rect.set_y(layout_constants::MEMORY_START_Y - OUTLINE);
         rect.set_height(layout_constants::MEMORY_HEIGHT + OUTLINE as u32);
         self.canvas.fill_rect(rect);
         
         self.canvas.set_draw_color(*layout_constants::BRIGHT_OUTLINE);
         //HORIZONTAL Bright
         rect.set_x(layout_constants::EDGE_SIZE);
         rect.set_y(layout_constants::EDGE_SIZE + layout_constants::GAME_HEIGHT as i32+ OUTLINE);
         rect.set_width(layout_constants::GAME_WIDTH + OUTLINE as u32);
         rect.set_height(OUTLINE as u32);
         self.canvas.fill_rect(rect);
 
         rect.set_x(layout_constants::EDGE_SIZE * 2 + OUTLINE * 2 + layout_constants::GAME_WIDTH as i32);
         rect.set_width(layout_constants::OPCODE_WIDTH + 2 * OUTLINE as u32);
         self.canvas.fill_rect(rect);
 
         rect.set_x(layout_constants::INFO_START_X - OUTLINE);
         rect.set_y(layout_constants::INFO_START_Y + layout_constants::INFO_HEIGHT as i32);
         rect.set_width(layout_constants::INFO_WIDTH + 2 * OUTLINE as u32);
         self.canvas.fill_rect(rect);

         rect.set_x(layout_constants::STACK_START_X - OUTLINE);
         rect.set_y(layout_constants::STACK_START_Y + layout_constants::STACK_HEIGHT as i32);
         rect.set_width(layout_constants::STACK_WIDTH + 2 * OUTLINE as u32);
         self.canvas.fill_rect(rect);
 
         rect.set_x(layout_constants::MEMORY_START_X - OUTLINE);
         rect.set_y(layout_constants::MEMORY_START_Y + layout_constants::MEMORY_HEIGHT as i32);
         rect.set_width(layout_constants::MEMORY_WIDTH + 2 * OUTLINE as u32);
         self.canvas.fill_rect(rect);
 
         //VERTIKAL Bright
         rect.set_x(GAME_START_X + layout_constants::GAME_WIDTH as i32);
         rect.set_y(layout_constants::EDGE_SIZE);
         rect.set_width(OUTLINE as u32);
         rect.set_height(layout_constants::GAME_HEIGHT + 2 * OUTLINE as u32);
         self.canvas.fill_rect(rect);
 
         rect.set_x(layout_constants::EDGE_SIZE * 2 + layout_constants::GAME_WIDTH as i32 + OUTLINE * 3 + layout_constants::OPCODE_WIDTH as i32);
         self.canvas.fill_rect(rect);
 
         rect.set_x(layout_constants::INFO_START_X + layout_constants::INFO_WIDTH as i32);
         rect.set_y(layout_constants::INFO_START_Y - OUTLINE);
         rect.set_height(layout_constants::INFO_HEIGHT + OUTLINE as u32);
         self.canvas.fill_rect(rect);

         rect.set_x(layout_constants::STACK_START_X + layout_constants::STACK_WIDTH as i32);
         rect.set_y(layout_constants::STACK_START_Y - OUTLINE);
         rect.set_height(layout_constants::STACK_HEIGHT + OUTLINE as u32);
         self.canvas.fill_rect(rect);
 
         rect.set_x(layout_constants::MEMORY_START_X + layout_constants::MEMORY_WIDTH as i32);
         rect.set_y(layout_constants::MEMORY_START_Y - OUTLINE);
         rect.set_height(layout_constants::MEMORY_HEIGHT + OUTLINE as u32);
         self.canvas.fill_rect(rect);
    }

    pub fn draw(&mut self) {
        self.draw_window();
        self.draw_outline();

        for display in self.displays.iter_mut() {
            display.as_mut().update_info();
            display.as_mut().redraw(&mut self.canvas, &mut self.ttf_context);
        }
        
        self.canvas.present();
    }

    fn draw_pixels(&mut self, pixels: [u8; COLUMNS * ROWS]) {
        let mut rect = rect::Rect::new(GAME_START_X, GAME_START_Y,
             PIXEL_SCALE as u32, PIXEL_SCALE as u32); 
        for y in 0..ROWS {
            rect.set_y((y * PIXEL_SCALE) as i32 + GAME_START_Y);
            for x in 0..COLUMNS {
                if pixels[(y * COLUMNS) + x] == 1 {
                    self.canvas.set_draw_color(Color::RGB(170, 255, 170));
                } else {
                    self.canvas.set_draw_color(Color::RGB(40, 40, 40));
                }
                rect.set_x((x * PIXEL_SCALE) as i32 + GAME_START_X);
                self.canvas.fill_rect(rect).expect("Error: could not draw pixel");
            }
        }
    }

    pub fn get_quit(&mut self) -> bool {
        self.quit
    }
}