use crate::display::{layout_constants::*};
use sdl2::{
    render::WindowCanvas, rect::Rect};

pub struct WindowRenderer {

}

impl WindowRenderer {
    pub fn new() -> WindowRenderer {
        WindowRenderer {

        }
    }

    pub fn render_background(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(*WINDOW_BACKGROUND);
        let rect = Rect::new(WINDOW_START_X, WINDOW_START_Y, WINDOW_WIDTH, WINDOW_HEIGHT); 
        canvas.fill_rect(rect)?;

        Ok(())
    }

    pub fn render_outline(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let mut rect = Rect::new(EDGE_SIZE + OUTLINE, 
            EDGE_SIZE, 
            GAME_WIDTH, 
            OUTLINE as u32);

         //HORIZONTAL Dark
         canvas.set_draw_color(*DARK_OUTLINE);
         canvas.fill_rect(rect)?;
 
         rect.set_x(OPCODE_START_X);
         rect.set_width(OPCODE_WIDTH);
         canvas.fill_rect(rect)?;
 
         rect.set_x(EDGE_SIZE);
         rect.set_y(INFO_START_Y - OUTLINE);
         rect.set_width(INFO_WIDTH + OUTLINE as u32);
         canvas.fill_rect(rect)?;

         rect.set_x(KEYPAD_START_X - OUTLINE);
         rect.set_y(KEYPAD_START_Y - OUTLINE);
         rect.set_width(KEYPAD_WIDTH + OUTLINE as u32);
         canvas.fill_rect(rect)?;

         rect.set_x(STACK_START_X - OUTLINE);
         rect.set_y(STACK_START_Y - OUTLINE);
         rect.set_width(STACK_WIDTH + OUTLINE as u32);
         canvas.fill_rect(rect)?;
 
         rect.set_x(MEMORY_START_X - OUTLINE);
         rect.set_y(MEMORY_START_Y - OUTLINE);
         rect.set_width(MEMORY_WIDTH + OUTLINE as u32);
         canvas.fill_rect(rect)?;
 
         //VERTICAL Dark
         rect.set_x(EDGE_SIZE);
         rect.set_y(EDGE_SIZE);
         rect.set_width(OUTLINE as u32);
         rect.set_height(GAME_HEIGHT + OUTLINE as u32);
         canvas.fill_rect(rect)?;
 
         rect.set_x(OPCODE_START_X - OUTLINE);
         canvas.fill_rect(rect)?;
 
         rect.set_x(EDGE_SIZE);
         rect.set_y(INFO_START_Y - OUTLINE);
         rect.set_height(INFO_HEIGHT + OUTLINE as u32);
         canvas.fill_rect(rect)?;

         rect.set_x(KEYPAD_START_X - OUTLINE);
         rect.set_y(KEYPAD_START_Y - OUTLINE);
         rect.set_height(KEYPAD_HEIGHT + OUTLINE as u32);
         canvas.fill_rect(rect)?;

         rect.set_x(STACK_START_X - OUTLINE);
         rect.set_y(STACK_START_Y - OUTLINE);
         rect.set_height(STACK_HEIGHT + OUTLINE as u32);
         canvas.fill_rect(rect)?;
 
         rect.set_x(MEMORY_START_X - OUTLINE);
         rect.set_y(MEMORY_START_Y - OUTLINE);
         rect.set_height(MEMORY_HEIGHT + OUTLINE as u32);
         canvas.fill_rect(rect)?;
         
         canvas.set_draw_color(*BRIGHT_OUTLINE);
         //HORIZONTAL Bright
         rect.set_x(EDGE_SIZE);
         rect.set_y(EDGE_SIZE + GAME_HEIGHT as i32+ OUTLINE);
         rect.set_width(GAME_WIDTH + OUTLINE as u32);
         rect.set_height(OUTLINE as u32);
         canvas.fill_rect(rect)?;
 
         rect.set_x(EDGE_SIZE * 2 + OUTLINE * 2 + GAME_WIDTH as i32);
         rect.set_width(OPCODE_WIDTH + 2 * OUTLINE as u32);
         canvas.fill_rect(rect)?;
 
         rect.set_x(INFO_START_X - OUTLINE);
         rect.set_y(INFO_START_Y + INFO_HEIGHT as i32);
         rect.set_width(INFO_WIDTH + 2 * OUTLINE as u32);
         canvas.fill_rect(rect)?;

         rect.set_x(KEYPAD_START_X - OUTLINE);
         rect.set_y(KEYPAD_START_Y + KEYPAD_HEIGHT as i32);
         rect.set_width(KEYPAD_WIDTH + 2 * OUTLINE as u32);
         canvas.fill_rect(rect)?;

         rect.set_x(STACK_START_X - OUTLINE);
         rect.set_y(STACK_START_Y + STACK_HEIGHT as i32);
         rect.set_width(STACK_WIDTH + 2 * OUTLINE as u32);
         canvas.fill_rect(rect)?;
 
         rect.set_x(MEMORY_START_X - OUTLINE);
         rect.set_y(MEMORY_START_Y + MEMORY_HEIGHT as i32);
         rect.set_width(MEMORY_WIDTH + 2 * OUTLINE as u32);
         canvas.fill_rect(rect)?;
 
         //VERTIKAL Bright
         rect.set_x(GAME_START_X + GAME_WIDTH as i32);
         rect.set_y(EDGE_SIZE);
         rect.set_width(OUTLINE as u32);
         rect.set_height(GAME_HEIGHT + 2 * OUTLINE as u32);
         canvas.fill_rect(rect)?;
 
         rect.set_x(EDGE_SIZE * 2 + GAME_WIDTH as i32 + OUTLINE * 3 + OPCODE_WIDTH as i32);
         canvas.fill_rect(rect)?;
 
         rect.set_x(INFO_START_X + INFO_WIDTH as i32);
         rect.set_y(INFO_START_Y - OUTLINE);
         rect.set_height(INFO_HEIGHT + OUTLINE as u32);
         canvas.fill_rect(rect)?;

         rect.set_x(KEYPAD_START_X + KEYPAD_WIDTH as i32);
         rect.set_y(KEYPAD_START_Y - OUTLINE);
         rect.set_height(KEYPAD_HEIGHT + OUTLINE as u32);
         canvas.fill_rect(rect)?;

         rect.set_x(STACK_START_X + STACK_WIDTH as i32);
         rect.set_y(STACK_START_Y - OUTLINE);
         rect.set_height(STACK_HEIGHT + OUTLINE as u32);
         canvas.fill_rect(rect)?;
 
         rect.set_x(MEMORY_START_X + MEMORY_WIDTH as i32);
         rect.set_y(MEMORY_START_Y - OUTLINE);
         rect.set_height(MEMORY_HEIGHT + OUTLINE as u32);
         canvas.fill_rect(rect)?;

         Ok(())
    }
}