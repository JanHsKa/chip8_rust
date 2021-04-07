use crate::display::layout_constants;
use crate::processor::MemoryAccess;
use crate::interfaces::IDisplay;
use crate::processor::memory_constants::{ROWS, COLUMNS, GRAPHIC_SIZE};

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
    access: Rc<RefCell<MemoryAccess>>,
    pixel_state: [u8; GRAPHIC_SIZE],
}

impl GameDisplay {
    pub fn new(mem_access: Rc<RefCell<MemoryAccess>>) -> GameDisplay {
        let array = mem_access.borrow_mut().get_graphic_array();
        GameDisplay {
            access: mem_access,
            pixel_state: array,
        }
    }
}

impl IDisplay for GameDisplay {
    fn update_info(&mut self) {
        self.pixel_state = self.access.borrow_mut().get_graphic_array();
    }

    fn redraw(&mut self, canvas: &mut WindowCanvas, _ttf_context: &mut Sdl2TtfContext) -> Result<(), String> {
        let mut rect = rect::Rect::new(GAME_START_X, GAME_START_Y , PIXEL_SCALE as u32, PIXEL_SCALE as u32); 
        for y in 0..ROWS {
            rect.set_y((y * PIXEL_SCALE) as i32 + GAME_START_Y);
            for x in 0..COLUMNS {
                if self.pixel_state[(y * COLUMNS) + x] == 1 {
                    canvas.set_draw_color(*layout_constants::GAME_PIXEL_SET);
                } else {
                    canvas.set_draw_color(*layout_constants::GAME_PIXEL_UNSET);
                }
                rect.set_x((x * PIXEL_SCALE) as i32 + GAME_START_X);
                canvas.fill_rect(rect)?;
            }
        }

        Ok(())
    }
}
