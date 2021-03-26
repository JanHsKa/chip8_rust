use crate::sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct GameDisplay {
    sdl_context: sdl2::Sdl,
}

impl GameDisplay {

    pub fn new() -> GameDisplay {
        GameDisplay {
            sdl_context: sdl2::init().unwrap(),
        }
    }

    pub fn initialize(&mut self) {
        
        let video = self.sdl_context.video().unwrap();
        let window = video.window("test window", 800, 600)
            .position_centered()
            .build()
            .expect("could not init");

        let mut canvas = window.into_canvas().build()
            .expect("could not init canvas");

        canvas.set_draw_color(Color::BLUE);
        canvas.clear();
        canvas.present(); 

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    _ => {}
                }
            }
        }
    }
}