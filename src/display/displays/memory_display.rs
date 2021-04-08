use crate::interfaces::IDisplay;
use crate::display::{
    layout_constants::{
        MEMORY_START_X, MEMORY_START_Y, 
        MEMORY_WIDTH, MEMORY_HEIGHT}, 
        DisplayRenderHelper};
use crate::processor::{MemoryAccess, memory_constants::{VARIABLES_COUNT}};
use std::{
    rc::Rc, cell::RefCell,
     result::Result, thread, time::Duration, 
    sync::{Arc, Mutex, mpsc::{
        Sender, Receiver, channel}}};

use sdl2::{
    ttf::Sdl2TtfContext, 
    render::{WindowCanvas}};

pub struct MemoryDisplay {
    variable_register: Vec<String>,
    remaining_register: Vec<String>,
    memory_access: Arc<Mutex<MemoryAccess>>,
    render_helper: DisplayRenderHelper,
}

impl IDisplay for MemoryDisplay {
    fn update_info(&mut self) {
        let mut access = self.memory_access.lock().unwrap();  
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

    fn redraw(&mut self, canvas: &mut WindowCanvas, ttf_context: &mut Sdl2TtfContext) -> Result<(), String> {
        self.render_helper.draw_lines(&mut self.variable_register, canvas, ttf_context)?;
        let start_x: i32 = MEMORY_START_X + MEMORY_WIDTH as i32 / 2;
        self.render_helper.draw_lines_with_x(&mut self.remaining_register, canvas, ttf_context, start_x)?;

        Ok(())
    }
}

impl MemoryDisplay {
    pub fn new(new_memory_access: Arc<Mutex<MemoryAccess>>) -> MemoryDisplay {
        let display_text: Vec<String> = vec![String::with_capacity(6); VARIABLES_COUNT];

        MemoryDisplay {
            variable_register: display_text,
            remaining_register: vec![String::with_capacity(6); 6],
            memory_access: new_memory_access,
            render_helper: DisplayRenderHelper::new(
                MEMORY_START_X, MEMORY_START_Y, 
                MEMORY_WIDTH, MEMORY_HEIGHT),
        }
    }
}


