
use crate::interfaces::IDisplay;
use crate::utils::{ProgramManager, ProgramState};
use crate::display::{FONTPATH1, FONTPATH2, FONTPATH3, FONTPATH4, FONTSIZE,
    layout_constants::{MEMORY_START_X, MEMORY_START_Y, MEMORY_WIDTH, LINE_PADDING, HIGHLIGHT_PADDING}
};
use crate::processor::{MemoryAccess, memory_constants::{VARIABLES_COUNT}};
use std::rc::Rc;
use std::collections::HashMap;
use sdl2::rect::Rect;
use std::cell::RefCell;
use sdl2::ttf::Font;
use sdl2::render::{TextureQuery, TextureCreator, WindowCanvas};
use sdl2::pixels::Color;
use sdl2::video::WindowContext;
use sdl2::surface::Surface;

pub struct MemoryDisplay {
    variable_register: Vec<String>,
    remaining_register: Vec<String>,
    memory_access: Rc<RefCell<MemoryAccess>>,
}

impl IDisplay for MemoryDisplay {
    fn update_info(&mut self) {
        let mut access = self.memory_access.borrow_mut();  
        let variables = access.get_variable_register();
        let register_index = VARIABLES_COUNT - 1;

        for (i, iter) in self.variable_register.iter_mut().enumerate() {
            *iter = format!("V{:X}: {:02X}", register_index - i, variables[register_index - i]);
        }

        self.remaining_register[0] = format!("PC:  {:04X}", access.get_program_counter());
        self.remaining_register[1] = format!("IR:  {:04X}", access.get_index_register());
        self.remaining_register[2] = format!("SP:  {:03X}", access.get_stack_pointer());
        self.remaining_register[3] = " ".to_string();
        self.remaining_register[4] = format!("DT:  {:02X}", access.get_delay_timer());
        self.remaining_register[5] = format!("ST:  {:02X}", access.get_sound_timer());
    }

    fn redraw(&mut self, canvas: &mut WindowCanvas, ttf_context: &mut sdl2::ttf::Sdl2TtfContext) {
        let mut font = ttf_context.load_font(FONTPATH4, FONTSIZE).unwrap();
        //font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = canvas.texture_creator();
        let mut lines_to_draw = self.variable_register.clone();
        for (i, iter) in lines_to_draw.iter_mut().enumerate() {
            self.render_text_line(canvas, &font, &texture_creator, iter, i, MEMORY_START_X);
        }

        lines_to_draw = self.remaining_register.clone();

        for (i, iter) in lines_to_draw.iter_mut().enumerate() {
            self.render_text_line(canvas, &font, &texture_creator, iter, i, MEMORY_START_X + MEMORY_WIDTH as i32 / 2);
        }

    }
}

impl MemoryDisplay {
    pub fn new(new_memory_access: Rc<RefCell<MemoryAccess>>) -> MemoryDisplay {
        let display_text: Vec<String> = vec![String::with_capacity(6); VARIABLES_COUNT];

        MemoryDisplay {
            variable_register: display_text,
            remaining_register: vec![String::with_capacity(6); 6],
            memory_access: new_memory_access,
        }
    }

    fn render_text_line(&mut self, canvas: &mut WindowCanvas, font: &Font,
            texture_creator: &TextureCreator<WindowContext>, text: &mut String, row: usize, start_x: i32) {

        let surface = font
            .render((*text).as_str())
            .blended(Color::WHITE)
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
    
        let TextureQuery { width, height, .. } = texture.query();
    
        let target = Rect::new(
            start_x + LINE_PADDING,
            MEMORY_START_Y + LINE_PADDING + ((FONTSIZE + LINE_PADDING as u16) * row as u16) as i32,
            width,
            height,
        );

        canvas.copy(&texture, None, target);
    }
}


