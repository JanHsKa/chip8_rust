use crate::interfaces::IDisplay;
use crate::utils::{ProgramManager, ProgramState};
use crate::display::{FONTPATH1, FONTPATH2, FONTPATH4, FONTSIZE,
    layout_constants::{STACK_START_X, STACK_START_Y, PADDING}
};
use crate::processor::{MemoryAccess, memory_constants::{STACKSIZE}};
use std::rc::Rc;
use std::collections::HashMap;
use sdl2::rect::Rect;
use std::cell::RefCell;
use sdl2::ttf::Font;
use sdl2::render::{TextureQuery, TextureCreator, WindowCanvas};
use sdl2::pixels::Color;
use sdl2::video::WindowContext;
use sdl2::surface::Surface;
//Controls: 
// F5: contninue / stop
// F6: step 
// +/-: speed
// F7: breakpoint
// F8: (maybe) step into
// F9:: Memory dump
// F1: restart
// F3: Open program in Editor

pub struct StackDisplay {
    game_name: String,
    stack: Vec<String>,
    game_size: u32,
    memory_access: Rc<RefCell<MemoryAccess>>,
}

impl IDisplay for StackDisplay {
    fn update_info(&mut self) {
        let mut access = self.memory_access.borrow_mut();  
        let stack = access.get_stack();
        let stack_size = STACKSIZE - 1;

        for (i, iter) in self.stack.iter_mut().enumerate() {
            *iter = format!("Stack {:X}:{:04X}", stack_size - i, stack[stack_size - i]);
        }
    }

    fn redraw(&mut self, canvas: &mut WindowCanvas, ttf_context: &mut sdl2::ttf::Sdl2TtfContext) {
        let mut font = ttf_context.load_font(FONTPATH2, FONTSIZE).unwrap();
        //font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = canvas.texture_creator();
        let mut lines_to_draw = self.stack.clone();
        for (i, iter) in lines_to_draw.iter_mut().enumerate() {
            self.render_text_line(canvas, &font, &texture_creator, iter, i);
        }
    }
}

impl StackDisplay {
    pub fn new(new_memory_access: Rc<RefCell<MemoryAccess>>) -> StackDisplay {
        let mut display_text: Vec<String> = vec![String::new(); STACKSIZE];

        StackDisplay {
            game_name: String::new(),
            stack: display_text,
            game_size: 0,
            memory_access: new_memory_access,
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
            STACK_START_X + PADDING,
            STACK_START_Y + PADDING + ((FONTSIZE + PADDING as u16) * row as u16) as i32,
            width,
            height,
        );

        canvas.copy(&texture, None, target);
    }
}