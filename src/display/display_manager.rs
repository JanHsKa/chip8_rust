use crate::display::{
    layout_constants::{
        WINDOW_HEIGHT, WINDOW_WIDTH, WINDOW_NAME},
        WindowRenderer};
use crate::interfaces::IDisplay;

use crate::utils::{TimeManager, TimeTo, InputChecker};
use sdl2::{
    render, Sdl, rect, 
    ttf::{Sdl2TtfContext}, 
    ttf, surface::Surface, video::Window};

use std::{boxed::Box, result::Result, thread, time::Duration, 
    sync::{Arc, Mutex, mpsc::{Sender, Receiver, channel}}};

pub const FONTPATH1: &str = "Data/Font/PrintChar21.ttf";
pub const FONTPATH2: &str = "Data/Font/8-BIT WONDER.ttf";
pub const FONTPATH3: &str = "Data/Font/C64_Pro-STYLE.ttf";
pub const FONTPATH4: &str = "Data/Font/C64_Pro_Mono-STYLE.ttf";
pub const ICONPATH: &str = "Data/Icons/Chip8_icon_24_21.bmp";

pub const FONTSIZE_LINE: u16 = 18;
pub const FONTSIZE_KEYPAD: u16 = 22;



pub struct DisplayManager {
    canvas: render::Canvas<Window>,
    displays: Vec<Box<dyn IDisplay>>,
    ttf_context: Sdl2TtfContext,
    window_renderer: WindowRenderer,
}

unsafe impl Send for DisplayManager {}

impl DisplayManager {
    pub fn new(context: Arc<Sdl>) -> DisplayManager {
        let video = context.video().unwrap();
        let mut sdl_window = video.window(WINDOW_NAME, WINDOW_WIDTH, WINDOW_HEIGHT)
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
            displays: Vec::new(),
            ttf_context: ttf,
            window_renderer: WindowRenderer::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        self.window_renderer.render_background(&mut self.canvas)?;
        self.window_renderer.render_outline(&mut self.canvas)?;
        self.canvas.present();

        self.draw();
        
        Ok(())
    }

    pub fn add_display(&mut self, display: Box<dyn IDisplay>) {
        self.displays.push(display);
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.window_renderer.render_background(&mut self.canvas)?;
        self.window_renderer.render_outline(&mut self.canvas)?;

        for display in self.displays.iter_mut() {
            display.as_mut().update_info();
            display.as_mut().redraw(&mut self.canvas, &mut self.ttf_context)?;
        }
        
        self.canvas.present();

        Ok(())
    }
}