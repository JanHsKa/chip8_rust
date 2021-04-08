use crate::display::{
    layout_constants::{STACK_HEIGHT, STACK_START_X, STACK_START_Y, STACK_WIDTH},
    DisplayRenderHelper,
};
use crate::interfaces::IDisplay;
use crate::processor::{memory_constants::STACKSIZE, MemoryAccess};
use crate::utils::{ProgramManager, ProgramState};
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

use sdl2::{pixels::Color, render::WindowCanvas, ttf::Sdl2TtfContext};

pub struct StackDisplay {
    stack: Vec<String>,
    memory_access: Arc<Mutex<MemoryAccess>>,
    stack_pointer: usize,
    render_helper: DisplayRenderHelper,
}

impl IDisplay for StackDisplay {
    fn update_info(&mut self) {
        let mut access = self.memory_access.lock().unwrap();
        let stack = access.get_stack();
        let stack_size = STACKSIZE - 1;
        self.stack_pointer = access.get_stack_pointer();

        for (i, iter) in self.stack.iter_mut().enumerate() {
            *iter = format!("Stack {:X}: {:04X}", stack_size - i, stack[stack_size - i]);
        }
    }

    fn redraw(
        &mut self,
        canvas: &mut WindowCanvas,
        ttf_context: &mut Sdl2TtfContext,
    ) -> Result<(), String> {
        self.render_helper
            .draw_lines(&mut self.stack, canvas, ttf_context)?;

        let y = STACKSIZE - self.stack_pointer - 1;
        self.render_helper
            .draw_rectangle(canvas, y as i32, Color::RED)?;

        Ok(())
    }
}

impl StackDisplay {
    pub fn new(new_memory_access: Arc<Mutex<MemoryAccess>>) -> StackDisplay {
        let display_text: Vec<String> = vec![String::new(); STACKSIZE];

        StackDisplay {
            stack: display_text,
            memory_access: new_memory_access,
            stack_pointer: 0,
            render_helper: DisplayRenderHelper::new(
                STACK_START_X,
                STACK_START_Y,
                STACK_WIDTH,
                STACK_HEIGHT,
            ),
        }
    }
}
