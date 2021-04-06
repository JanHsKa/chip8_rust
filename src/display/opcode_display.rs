use crate::interfaces::IDisplay;
use crate::utils::{ProgramManager, ProgramState};
use crate::display::{FONTPATH1, FONTPATH2, FONTPATH3, FONTPATH4, FONTSIZE,
    layout_constants::{OPCODE_START_X, OPCODE_START_Y, OPCODE_WIDTH, OPCODE_LINES, LINE_PADDING, HIGHLIGHT_PADDING}
};
use crate::processor::{MemoryAccess, memory_constants::{STACKSIZE, PROGRAM_START}};
use std::rc::Rc;
use std::collections::HashMap;
use sdl2::rect::Rect;
use std::cell::RefCell;
use sdl2::ttf::Font;
use sdl2::render::{TextureQuery, TextureCreator, WindowCanvas};
use sdl2::pixels::Color;
use sdl2::video::WindowContext;
use sdl2::surface::Surface;

pub struct OpcodeDisplay {
    code_lines: Vec<String>,
    memory_access: Rc<RefCell<MemoryAccess>>,
    program_manager: Rc<RefCell<ProgramManager>>,
    offset: usize,
    current_line: usize,
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

    fn redraw(&mut self, canvas: &mut WindowCanvas, ttf_context: &mut sdl2::ttf::Sdl2TtfContext) {
        let mut font = ttf_context.load_font(FONTPATH4, FONTSIZE).unwrap();
        //font.set_style(sdl2::ttf::FontStyle::BOLD);

        let mut rect_y: i32 = (self.current_line - self.offset) as i32 / 2; 
        if rect_y == OPCODE_LINES as i32 {
            rect_y -= 1;
        }

        let rectangle = Rect::new(
            OPCODE_START_X,
            OPCODE_START_Y + HIGHLIGHT_PADDING + rect_y * (FONTSIZE as i32 + LINE_PADDING),
             OPCODE_WIDTH, 
             2 * (LINE_PADDING / 2) as u32 + FONTSIZE as u32);

        canvas.set_draw_color(Color::RGB(51, 51, 255));
        canvas.fill_rect(rectangle);

        let texture_creator = canvas.texture_creator();
        let mut lines_to_draw = self.code_lines.clone();
        for (i, iter) in lines_to_draw.iter_mut().enumerate() {
            self.render_text_line(canvas, &font, &texture_creator, iter, i);
        }
    }
}

impl OpcodeDisplay {
    pub fn new(new_memory_access: Rc<RefCell<MemoryAccess>>, new_program_manager: Rc<RefCell<ProgramManager>>) -> OpcodeDisplay {
        let display_text: Vec<String> = vec![String::with_capacity(10); OPCODE_LINES];

        OpcodeDisplay {
            code_lines: display_text,
            memory_access: new_memory_access,
            program_manager: new_program_manager,
            offset: 0,
            current_line: 0,
        }
    }

    fn render_text_line(&mut self, canvas: &mut WindowCanvas, font: &Font,
            texture_creator: &TextureCreator<WindowContext>, text: &mut String, row: usize) {

        let surface = font
            .render((*text).as_str())
            .blended(Color::WHITE)
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
    
        let TextureQuery { width, height, .. } = texture.query();
    
        let target = Rect::new(
            OPCODE_START_X + LINE_PADDING,
            OPCODE_START_Y + LINE_PADDING + ((FONTSIZE + LINE_PADDING as u16) * row as u16) as i32,
            width,
            height,
        );

        canvas.copy(&texture, None, target);
    } 

    fn update_offset(&mut self) {
    }
}