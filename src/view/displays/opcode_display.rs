use crate::defines::{
    layout_constants::{
        OPCODE_HEIGHT, OPCODE_HIGHLIGHT_TEST,
        OPCODE_LINES, OPCODE_START_X, OPCODE_START_Y, OPCODE_WIDTH,
    },
    memory_constants::PROGRAM_START,
    IDisplay,
};
use crate::model::{GamePropertiesAccess, MemoryAccess};
use crate::view::{Disassembler, DisplayRenderHelper};
use std::{
    collections::HashSet,
    result::Result,
    sync::{Arc, Mutex},
};

use sdl2::{pixels::Color, render::WindowCanvas, ttf::Sdl2TtfContext};

pub struct OpcodeDisplay {
    code_lines: Vec<String>,
    memory_access: Arc<Mutex<MemoryAccess>>,
    game_properties_access: Arc<Mutex<GamePropertiesAccess>>,
    offset: usize,
    current_line: usize,
    render_helper: DisplayRenderHelper,
    breakpoints: HashSet<usize>,
    highlight_color: Color,
}

impl IDisplay for OpcodeDisplay {
    fn update_info(&mut self) {
        let mut access = self.memory_access.lock().unwrap();
        let mut properties = self.game_properties_access.lock().unwrap();

        self.current_line = access.get_program_counter() - PROGRAM_START;
        let program_size = properties.get_game_size();

        if let Some(offset_change) = self.update_offset(program_size) {
            self.offset = offset_change;
            let start = self.offset;
            if let Some(code_snippet) = access.get_code_snippet(OPCODE_LINES / 2, self.offset) {
                let disassambled_code = Disassembler::disassemble_list(&code_snippet);

                for (i, iter) in self.code_lines.iter_mut().enumerate() {
                    *iter = format!("{:004} - ", start + i * 2);
                    iter.push_str(&disassambled_code[i]);
                }
            } else {
                for (_i, iter) in self.code_lines.iter_mut().enumerate() {
                    *iter = "0000".to_string();
                }
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
            .fill_rectangle(canvas, rect_y, self.highlight_color)?;
        self.render_helper
            .draw_lines(&mut self.code_lines, canvas, ttf_context)?;

        Ok(())
    }
}

impl OpcodeDisplay {
    pub fn new(
        new_memory_access: Arc<Mutex<MemoryAccess>>,
        new_program_manager: Arc<Mutex<GamePropertiesAccess>>,
    ) -> OpcodeDisplay {
        let display_text: Vec<String> = vec![String::with_capacity(10); OPCODE_LINES / 2];

        OpcodeDisplay {
            code_lines: display_text,
            memory_access: new_memory_access,
            game_properties_access: new_program_manager,
            offset: 0,
            current_line: 0,
            render_helper: DisplayRenderHelper::new(
                OPCODE_START_X,
                OPCODE_START_Y,
                OPCODE_WIDTH,
                OPCODE_HEIGHT,
            ),
            breakpoints: HashSet::new(),
            highlight_color: OPCODE_HIGHLIGHT_TEST,
        }
    }

    fn update_offset(&self, program_size: usize) -> Option<usize> {
        if self.current_line >= self.offset + OPCODE_LINES {
            if self.current_line + OPCODE_LINES >= program_size {
                return Some(program_size - OPCODE_LINES - 2);
            } else {
                return Some(self.current_line - OPCODE_LINES + 2);
            }
        } else if self.current_line < self.offset {
            return Some(self.current_line);
        }

        None
    }

    fn update_breakpoints(&self) {}
}
