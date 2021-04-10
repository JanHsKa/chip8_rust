use crate::controller::ProgramManager;
use crate::defines::{
    layout_constants::{
        OPCODE_HEIGHT, OPCODE_HIGHLIGHT_NORMAL, OPCODE_HIGHLIGHT_TEST, OPCODE_LINES,
        OPCODE_START_X, OPCODE_START_Y, OPCODE_WIDTH,
    },
    memory_constants::PROGRAM_START,
    IDisplay,
};
use crate::model::MemoryAccess;
use crate::view::DisplayRenderHelper;
use std::{
    cell::RefCell,
    rc::Rc,
    result::Result,
    sync::{Arc, Mutex},
};

use sdl2::{pixels::Color, render::WindowCanvas, ttf::Sdl2TtfContext};

pub struct OpcodeDisplay {
    code_lines: Vec<String>,
    memory_access: Arc<Mutex<MemoryAccess>>,
    program_manager: Arc<Mutex<ProgramManager>>,
    offset: usize,
    current_line: usize,
    render_helper: DisplayRenderHelper,
}

impl IDisplay for OpcodeDisplay {
    fn update_info(&mut self) {
        let mut access = self.memory_access.lock().unwrap();
        let mut manager = self.program_manager.lock().unwrap();

        self.current_line = access.get_program_counter() - PROGRAM_START;
        let program_size = manager.get_program_size();

        if self.current_line > self.offset + OPCODE_LINES * 2 {
            println!(" greater");
            if self.current_line + OPCODE_LINES * 2 >= program_size {
                println!("max");
                self.offset = program_size - OPCODE_LINES * 2 - 1;
            } else {
                println!("highger");

                self.offset = self.current_line - OPCODE_LINES * 2;
            }
        } else if self.current_line < self.offset {
            self.offset = self.current_line;
        }

        let start = self.offset;
        if let Some(code_snippet) = manager.get_code_snippet(OPCODE_LINES, self.offset) {
            for (i, iter) in self.code_lines.iter_mut().enumerate() {
                *iter = format!("{}  -  {:04X}", start + i * 2, code_snippet[i]);
            }
            //println!("current line  {}", self.current_line);
            //println!("offset  {}", self.offset);

            //println!("first line {}", self.code_lines[0]);
            //println!("last line {}", self.code_lines[OPCODE_LINES - 1]);
        } else {
            for (i, iter) in self.code_lines.iter_mut().enumerate() {
                *iter = "0000".to_string();
            }
        }
    }

    fn redraw(
        &mut self,
        canvas: &mut WindowCanvas,
        ttf_context: &mut Sdl2TtfContext,
    ) -> Result<(), String> {
        let mut rect_y: i32 = (self.current_line - self.offset) as i32 / 2;
        if rect_y == OPCODE_LINES as i32 {
            rect_y -= 1;
        }

        self.render_helper
            .fill_rectangle(canvas, rect_y, OPCODE_HIGHLIGHT_TEST)?;
        self.render_helper
            .draw_lines(&mut self.code_lines, canvas, ttf_context)?;

        Ok(())
    }
}

impl OpcodeDisplay {
    pub fn new(
        new_memory_access: Arc<Mutex<MemoryAccess>>,
        new_program_manager: Arc<Mutex<ProgramManager>>,
    ) -> OpcodeDisplay {
        let display_text: Vec<String> = vec![String::with_capacity(10); OPCODE_LINES];

        OpcodeDisplay {
            code_lines: display_text,
            memory_access: new_memory_access,
            program_manager: new_program_manager,
            offset: 0,
            current_line: 0,
            render_helper: DisplayRenderHelper::new(
                OPCODE_START_X,
                OPCODE_START_Y,
                OPCODE_WIDTH,
                OPCODE_HEIGHT,
            ),
        }
    }
}
