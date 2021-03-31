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

pub const WINDOW_WIDTH: u32 = 1350;
pub const WINDOW_HEIGHT: u32 = 850;

pub const GAME_WIDTH: u32 = (constants::COLUMNS * PIXEL_SCALE) as u32;
pub const GAME_HEIGHT: u32 = (constants::ROWS * PIXEL_SCALE) as u32;
pub const GAME_START_X: i32 = EDGE_SIZE + OUTLINE;
pub const GAME_START_Y: i32 = EDGE_SIZE + OUTLINE;

pub const MEMORY_HEIGHT: u32 = WINDOW_HEIGHT - 3 * EDGE_SIZE as u32 - 4 * OUTLINE as u32 - GAME_HEIGHT;
pub const MEMORY_WIDTH: u32 = OPCODE_WIDTH;
pub const MEMORY_START_X: i32 = OPCODE_START_X;
pub const MEMORY_START_Y: i32 = 2 * EDGE_SIZE + 3 * OUTLINE + GAME_HEIGHT as i32;

pub const CONTROL_HEIGHT: u32 = MEMORY_HEIGHT;
pub const CONTROL_WIDTH: u32 = MEMORY_WIDTH;
pub const CONTROL_START_X: i32 = EDGE_SIZE + OUTLINE;
pub const CONTROL_START_Y: i32 = 2 * EDGE_SIZE + GAME_HEIGHT as i32 + 3 * OUTLINE;

pub const OPCODE_WIDTH: u32 = WINDOW_WIDTH - 3 * EDGE_SIZE as u32 - 4 * OUTLINE as u32 - GAME_WIDTH;
pub const OPCODE_HEIGHT: u32 = GAME_HEIGHT;
pub const OPCODE_START_X: i32 = EDGE_SIZE * 2 + OUTLINE * 3 + GAME_WIDTH as i32;
pub const OPCODE_START_y: i32 = GAME_START_Y;







