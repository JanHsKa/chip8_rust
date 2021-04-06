use crate::interfaces::IDisplay;
use crate::utils::{ProgramManager, ProgramState};
use crate::display::{FONTPATH1, FONTPATH2, FONTPATH3, FONTPATH4, FONTSIZE,
    layout_constants::{STACK_START_X, STACK_START_Y, STACK_WIDTH, LINE_PADDING, HIGHLIGHT_PADDING}
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

pub struct StackDisplay {
    stack: Vec<String>,
    memory_access: Rc<RefCell<MemoryAccess>>,
    stack_pointer: usize,
}

impl IDisplay for StackDisplay {
    fn update_info(&mut self) {
        let mut access = self.memory_access.borrow_mut();  
        let stack = access.get_stack();
        let stack_size = STACKSIZE - 1;
        self.stack_pointer = access.get_stack_pointer();

        for (i, iter) in self.stack.iter_mut().enumerate() {
            *iter = format!("Stack {:X}: {:04X}", stack_size - i, stack[stack_size - i]);
        }
    }

    fn redraw(&mut self, canvas: &mut WindowCanvas, ttf_context: &mut sdl2::ttf::Sdl2TtfContext) {
        let mut font = ttf_context.load_font(FONTPATH4, FONTSIZE).unwrap();
        //font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = canvas.texture_creator();
        let mut lines_to_draw = self.stack.clone();
        for (i, iter) in lines_to_draw.iter_mut().enumerate() {
            self.render_text_line(canvas, &font, &texture_creator, iter, i);
        }


        let rectangle = Rect::new(
            STACK_START_X,
            STACK_START_Y + HIGHLIGHT_PADDING + (STACKSIZE - self.stack_pointer - 1) as i32 * (FONTSIZE as i32 + LINE_PADDING),
             STACK_WIDTH, 
             2 * (LINE_PADDING / 2) as u32 + FONTSIZE as u32);

        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(rectangle);
    }
}

impl StackDisplay {
    pub fn new(new_memory_access: Rc<RefCell<MemoryAccess>>) -> StackDisplay {
        let display_text: Vec<String> = vec![String::new(); STACKSIZE];

        StackDisplay {
            stack: display_text,
            memory_access: new_memory_access,
            stack_pointer: 0,
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
            STACK_START_X + LINE_PADDING,
            STACK_START_Y + LINE_PADDING + ((FONTSIZE + LINE_PADDING as u16) * row as u16) as i32,
            width,
            height,
        );

        canvas.copy(&texture, None, target);
    }
}