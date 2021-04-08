use crate::interfaces::IDisplay;
use crate::utils::{ProgramManager, ProgramState};
use crate::display::{
    layout_constants::{KEYPAD_START_Y, KEYPAD_START_X, 
        KEYPAD_WIDTH, KEYPAD_HEIGHT},
        KeypadRenderer};
        
use crate::processor::{MemoryAccess, memory_constants::{STACKSIZE}};
use std::{
    rc::Rc, cell::RefCell, sync::{Mutex, MutexGuard, Arc}};
use sdl2::{
    ttf::Sdl2TtfContext, 
    render::{WindowCanvas}, pixels::Color};

pub struct KeypadDisplay {
    stack: Vec<String>,
    program_manager: Rc<RefCell<ProgramManager>>,
    stack_pointer: usize,
    render_helper: KeypadRenderer,
}

impl IDisplay for KeypadDisplay {
    fn update_info(&mut self) {
       
    }

    fn redraw(&mut self, canvas: &mut WindowCanvas, ttf_context: &mut Sdl2TtfContext) -> Result<(), String> {
        self.render_helper.draw_lines(&mut self.stack, canvas, ttf_context)?;

        let y = STACKSIZE - self.stack_pointer - 1;
        //self.render_helper.draw_rectangle(canvas, y as i32, Color::RED)?;

        Ok(())
    }
}

impl KeypadDisplay {
    pub fn new(new_program_manager: Rc<RefCell<ProgramManager>>) -> KeypadDisplay {
        let display_text: Vec<String> = vec![String::new(); STACKSIZE];

        KeypadDisplay {
            stack: display_text,
            program_manager: new_program_manager,
            stack_pointer: 0,
            render_helper: KeypadRenderer::new(),
        }
    }
}