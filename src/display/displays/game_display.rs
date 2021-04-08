use crate::display::layout_constants;
use crate::processor::{MemoryAccess, Resolution,
    memory_constants::{ROWS, COLUMNS, GRAPHIC_SIZE}};
use crate::interfaces::IDisplay;

use sdl2::{
    render::WindowCanvas, rect, 
    ttf::Sdl2TtfContext};
use std::{
    rc::Rc, cell::RefCell};

use self::layout_constants::{
    PIXEL_SCALE, 
    GAME_START_X, 
    GAME_START_Y,
};


   
pub struct GameDisplay {
    memory_access: Rc<RefCell<MemoryAccess>>,
    pixel_state: Vec<u8>,
    resolution: Resolution,
    pixel_scale: usize,
}

impl GameDisplay {
    pub fn new(mem_access: Rc<RefCell<MemoryAccess>>) -> GameDisplay {
        let array = mem_access.borrow_mut().get_graphic_array();
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
        let mut access = self.memory_access.borrow_mut();
        self.pixel_state = access.get_graphic_array();
        self.resolution = access.get_resolution();
        self.pixel_scale = PIXEL_SCALE / self.resolution as usize;
    }

    fn redraw(&mut self, canvas: &mut WindowCanvas, _ttf_context: &mut Sdl2TtfContext) -> Result<(), String> {
        let mut rect = rect::Rect::new(
            GAME_START_X, GAME_START_Y , 
            self.pixel_scale as u32, self.pixel_scale as u32); 

        for y in 0..ROWS {
            rect.set_y((y * self.pixel_scale) as i32 + GAME_START_Y);
            for x in 0..COLUMNS {
                if self.pixel_state[(y * COLUMNS) + x] == 1 {
                    canvas.set_draw_color(*layout_constants::GAME_PIXEL_SET);
                } else {
                    canvas.set_draw_color(*layout_constants::GAME_PIXEL_UNSET);
                }
                rect.set_x((x * self.pixel_scale) as i32 + GAME_START_X);
                canvas.fill_rect(rect)?;
            }
        }

        Ok(())
    }
}
