use crate::interfaces::IDisplay;
use crate::utils::{ProgramManager};
use crate::display::{
    layout_constants::{
        OPCODE_START_X, OPCODE_START_Y, OPCODE_WIDTH, 
        OPCODE_HEIGHT, OPCODE_LINES}, 
        DisplayRenderHelper
};
use crate::processor::{MemoryAccess, memory_constants::{PROGRAM_START}};
use std::{
    rc::Rc, cell::RefCell};
use sdl2::{
    ttf::Sdl2TtfContext, 
    render::{WindowCanvas}, 
    pixels::Color};

pub struct OpcodeDisplay {
    code_lines: Vec<String>,
    memory_access: Rc<RefCell<MemoryAccess>>,
    program_manager: Rc<RefCell<ProgramManager>>,
    offset: usize,
    current_line: usize,
    render_helper: DisplayRenderHelper,
}

impl IDisplay for OpcodeDisplay {
    fn update_info(&mut self) {
        let mut access = self.memory_access.borrow_mut();  
        let mut manager = self.program_manager.borrow_mut();

        self.current_line = access.get_program_counter() - PROGRAM_START;
        let program_size = manager.get_program_size();
        
        if self.current_line > self.offset + OPCODE_LINES * 2{
            if self.current_line + OPCODE_LINES * 2 >= program_size {
                self.offset = program_size - OPCODE_LINES * 2 - 1;
            } else {
                self.offset = self.current_line - OPCODE_LINES * 2;
            }
        }  else if self.current_line < self.offset {
            self.offset = self.current_line;
        }

        let start = self.offset;
        if let Some(code_snippet) = manager.get_code_snippet(OPCODE_LINES, self.offset) {
            for (i, iter) in self.code_lines.iter_mut().enumerate() {
                *iter = format!("{:04X}  -  {:04X}", start / 2 + i, code_snippet[i]);
            }

        } else {
            for (i, iter) in self.code_lines.iter_mut().enumerate() {
                *iter = "0000".to_string();
            }
        }


    }

    fn redraw(&mut self, canvas: &mut WindowCanvas, ttf_context: &mut Sdl2TtfContext) -> Result<(), String> {
        let mut rect_y: i32 = (self.current_line - self.offset) as i32 / 2; 
        if rect_y == OPCODE_LINES as i32 {
            rect_y -= 1;
        }

        self.render_helper.fill_rectangle(canvas, rect_y, Color::RGB(51, 51, 255))?;
        self.render_helper.draw_lines(&mut self.code_lines, canvas, ttf_context)?;

        Ok(())
    }
}

impl OpcodeDisplay {
    pub fn new(new_memory_access: Rc<RefCell<MemoryAccess>>, 
        new_program_manager: Rc<RefCell<ProgramManager>>) -> OpcodeDisplay {

        let display_text: Vec<String> = vec![String::with_capacity(10); OPCODE_LINES];

        OpcodeDisplay {
            code_lines: display_text,
            memory_access: new_memory_access,
            program_manager: new_program_manager,
            offset: 0,
            current_line: 0,
            render_helper: DisplayRenderHelper::new(
                OPCODE_START_X, OPCODE_START_Y, 
                OPCODE_WIDTH, OPCODE_HEIGHT),
        }
    }
}