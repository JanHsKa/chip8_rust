use crate::defines::{
    layout_constants::{MEMORY_HEIGHT, MEMORY_START_X, MEMORY_START_Y, MEMORY_WIDTH},
    memory_constants::VARIABLES_COUNT,
};
use crate::interfaces::IDisplay;
use crate::model::MemoryAccess;
use crate::view::DisplayRenderHelper;
use std::{
    cell::RefCell,
    rc::Rc,
    result::Result,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use sdl2::{render::WindowCanvas, ttf::Sdl2TtfContext};

pub struct BreakPointDisplay {
    variable_register: Vec<String>,
    remaining_register: Vec<String>,
    memory_access: Arc<Mutex<MemoryAccess>>,
    render_helper: DisplayRenderHelper,
}

impl IDisplay for BreakPointDisplay {
    fn update_info(&mut self) {
        let mut access = self.memory_access.lock().unwrap();
        let variables = access.get_variable_register();
        let register_index = VARIABLES_COUNT - 1;

        for (i, iter) in self.variable_register.iter_mut().enumerate() {
            *iter = format!(
                "V{:X}: {:02X}",
                register_index - i,
                variables[register_index - i]
            );
        }

        self.remaining_register[0] = format!("PC:  {:04X}", access.get_program_counter());
        self.remaining_register[1] = format!("IR:  {:04X}", access.get_index_register());
        self.remaining_register[2] = format!("SP:  {:03X}", access.get_stack_pointer());
        self.remaining_register[3] = " ".to_string();
        self.remaining_register[4] = format!("DT:  {:02X}", access.get_delay_timer());
        self.remaining_register[5] = format!("ST:  {:02X}", access.get_sound_timer());
    }

    fn redraw(
        &mut self,
        canvas: &mut WindowCanvas,
        ttf_context: &mut Sdl2TtfContext,
    ) -> Result<(), String> {
        self.render_helper
            .draw_lines(&mut self.variable_register, canvas, ttf_context)?;
        let start_x: i32 = MEMORY_START_X + MEMORY_WIDTH as i32 / 2;
        self.render_helper.draw_lines_with_x(
            &mut self.remaining_register,
            canvas,
            ttf_context,
            start_x,
        )?;

        Ok(())
    }
}

impl BreakPointDisplay {
    pub fn new(new_memory_access: Arc<Mutex<MemoryAccess>>) -> BreakPointDisplay {
        let display_text: Vec<String> = vec![String::with_capacity(6); VARIABLES_COUNT];

        BreakPointDisplay {
            variable_register: display_text,
            remaining_register: vec![String::with_capacity(6); 6],
            memory_access: new_memory_access,
            render_helper: DisplayRenderHelper::new(
                MEMORY_START_X,
                MEMORY_START_Y,
                MEMORY_WIDTH,
                MEMORY_HEIGHT,
            ),
        }
    }
}
