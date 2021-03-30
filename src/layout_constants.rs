use sdl2::pixels::Color;
use crate::constants;

pub const WINDOW_BACKGROUND: Color = Color::RGB(25,35,45);  
pub const GAME_BACKGROUND: Color = Color::RGB(40, 40, 40);  
pub const GAME_FOREGROUND: Color = Color::RGB(170, 255, 170);
pub const DARK_OUTLINE: Color = Color::RGB(5, 10, 15);
pub const BRIGHT_OUTLINE: Color = Color::RGB(60, 80, 90);



pub const PIXEL_SCALE: usize = 15;
pub const EDGE_SIZE: i32 = 12;
pub const OUTLINE: i32 = 4;

pub const GAME_WIDTH: u32 = (constants::COLUMNS * PIXEL_SCALE) as u32;
pub const GAME_HEIGHT: u32 = (constants::ROWS * PIXEL_SCALE) as u32;
pub const GAME_START_X: i32 = EDGE_SIZE + OUTLINE;
pub const GAME_START_Y: i32 = EDGE_SIZE + OUTLINE;


pub const WINDOW_WIDTH: u32 = 1300;
pub const WINDOW_HEIGHT: u32 = 800;

pub const MEMORY_HEIGHT: u32 = WINDOW_HEIGHT - 4 * EDGE_SIZE as u32;
pub const MEMORY_WIDTH: u32 = 300;

pub const CONTROL_HEIGHT: u32 = MEMORY_HEIGHT;
pub const CONTROL_WIDTH: u32 = MEMORY_WIDTH;
pub const CONTROL_START_X: i32 = EDGE_SIZE + OUTLINE;
pub const CONTROL_START_Y: i32 = 2 * EDGE_SIZE + GAME_HEIGHT as i32;

pub const OPCODE_WIDTH: u32 = WINDOW_WIDTH - 3 * EDGE_SIZE as u32 - 4 * OUTLINE as u32;




