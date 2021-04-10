use crate::defines::{
    layout_constants::{WINDOW_HEIGHT, WINDOW_NAME, WINDOW_WIDTH},
    IDisplay,
};
use crate::view::{InputChecker, SoundManager, WindowRenderer};

use crate::controller::{TimeManager, TimeTo};
use sdl2::{rect, render, surface::Surface, ttf, ttf::Sdl2TtfContext, video::Window, Sdl};

use std::{
    boxed::Box,
    result::Result,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

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
    input_checker: InputChecker,
    update_receiver: Receiver<TimeTo>,
    sound_manager: SoundManager,
}

impl DisplayManager {
    pub fn new(
        context: Arc<Sdl>,
        new_input_checker: InputChecker,
        new_sound_manager: SoundManager,
    ) -> DisplayManager {
        let video = context.video().unwrap();
        let mut sdl_window = video
            .window(WINDOW_NAME, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .expect("Error: Could not init Window");

        let window_icon = Surface::load_bmp(ICONPATH).expect("Could not open icon");
        sdl_window.set_icon(window_icon);

        let new_canvas = sdl_window
            .into_canvas()
            .build()
            .expect("could not init canvas");

        let ttf = ttf::init().unwrap();

        let (new_sender, new_receiver) = channel();

        thread::spawn(move || {
            let mut time_manager = TimeManager::new(new_sender);
            time_manager.start_clock();
        });

        DisplayManager {
            canvas: new_canvas,
            displays: Vec::new(),
            ttf_context: ttf,
            input_checker: new_input_checker,
            update_receiver: new_receiver,
            sound_manager: new_sound_manager,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        WindowRenderer::render_background(&mut self.canvas)?;
        WindowRenderer::render_outline(&mut self.canvas)?;
        self.canvas.present();

        'running: loop {
            if self.check_for_redraw() {
                self.draw()?;
            }
            self.input_checker.check_input();
            self.sound_manager.check_sound();
            thread::sleep(Duration::from_micros(100));
        }
    }

    fn check_for_redraw(&mut self) -> bool {
        if let Ok(message) = self.update_receiver.try_recv() {
            return message == TimeTo::Update;
        }

        false
    }

    pub fn add_display(&mut self, display: Box<dyn IDisplay>) {
        self.displays.push(display);
    }

    pub fn draw(&mut self) -> Result<(), String> {
        WindowRenderer::render_background(&mut self.canvas)?;
        WindowRenderer::render_outline(&mut self.canvas)?;

        for display in self.displays.iter_mut() {
            display.as_mut().update_info();
            display
                .as_mut()
                .redraw(&mut self.canvas, &mut self.ttf_context)?;
        }

        self.canvas.present();

        Ok(())
    }
}
