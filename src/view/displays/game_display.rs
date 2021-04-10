use crate::defines::{
    layout_constants::{
        GAME_PIXEL_SET, GAME_PIXEL_TEST, GAME_PIXEL_UNSET, GAME_PIXEL_UNTEST, GAME_START_X,
        GAME_START_Y, PIXEL_SCALE,
    },
    memory_constants::{COLUMNS, GRAPHIC_SIZE, ROWS},
    IDisplay,
};
use crate::model::{MemoryAccess, Resolution};
use sdl2::{rect, render::WindowCanvas, ttf::Sdl2TtfContext};
use std::{
    cell::RefCell,
    rc::Rc,
    result::Result,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub struct GameDisplay {
    memory_access: Arc<Mutex<MemoryAccess>>,
    pixel_state: Vec<u8>,
    resolution: Resolution,
    pixel_scale: usize,
}

impl GameDisplay {
    pub fn new(mem_access: Arc<Mutex<MemoryAccess>>) -> GameDisplay {
        let array = mem_access.lock().unwrap().get_graphic_array();
        GameDisplay {
            memory_access: mem_access,
            pixel_state: array,
            resolution: Resolution::Low,
            pixel_scale: PIXEL_SCALE,
        }
    }
}

impl IDisplay for GameDisplay {
    fn update_info(&mut self) {
        let mut access = self.memory_access.lock().unwrap();
        self.pixel_state = access.get_graphic_array();
        self.resolution = access.get_resolution();
        self.pixel_scale = PIXEL_SCALE / self.resolution as usize;
    }

    fn redraw(
        &mut self,
        canvas: &mut WindowCanvas,
        _ttf_context: &mut Sdl2TtfContext,
    ) -> Result<(), String> {
        let mut rect = rect::Rect::new(
            GAME_START_X,
            GAME_START_Y,
            self.pixel_scale as u32,
            self.pixel_scale as u32,
        );

        for y in 0..ROWS {
            rect.set_y((y * self.pixel_scale) as i32 + GAME_START_Y);
            for x in 0..COLUMNS {
                if self.pixel_state[(y * COLUMNS) + x] == 1 {
                    canvas.set_draw_color(GAME_PIXEL_TEST);
                } else {
                    canvas.set_draw_color(GAME_PIXEL_UNSET);
                }
                rect.set_x((x * self.pixel_scale) as i32 + GAME_START_X);
                canvas.fill_rect(rect)?;
            }
        }

        Ok(())
    }
}
